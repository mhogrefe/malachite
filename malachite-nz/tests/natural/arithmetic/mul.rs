// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{CheckedLogBase2, DivMod, PowerOf2, Square};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::common::rle_decode;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{
    large_type_gen_var_1, unsigned_pair_gen_var_27, unsigned_vec_gen_var_6,
    unsigned_vec_pair_gen_var_1, unsigned_vec_pair_gen_var_2, unsigned_vec_triple_gen_var_1,
    unsigned_vec_triple_gen_var_2, unsigned_vec_triple_gen_var_24, unsigned_vec_triple_gen_var_25,
    unsigned_vec_triple_gen_var_3, unsigned_vec_unsigned_pair_gen,
    unsigned_vec_unsigned_unsigned_triple_gen, unsigned_vec_unsigned_vec_unsigned_triple_gen_var_1,
};
use malachite_base::vecs::vec_from_str;
use malachite_nz::natural::arithmetic::mul::fft::{
    limbs_mul_greater_to_out_fft, limbs_mul_greater_to_out_fft_scratch_len,
    limbs_mul_greater_to_out_fft_with_cutoff, limbs_mul_greater_to_out_fft_with_cutoff_scratch_len,
    limbs_square_to_out_fft, limbs_square_to_out_fft_scratch_len,
    limbs_square_to_out_fft_with_cutoff, limbs_square_to_out_fft_with_cutoff_scratch_len,
};
use malachite_nz::natural::arithmetic::mul::limb::{
    limbs_mul_limb, limbs_mul_limb_to_out, limbs_mul_limb_with_carry_to_out,
    limbs_slice_mul_limb_in_place, limbs_slice_mul_limb_with_carry_in_place,
    limbs_vec_mul_limb_in_place,
};
use malachite_nz::natural::arithmetic::mul::mul_low::{
    limbs_mul_low_same_length, limbs_mul_low_same_length_basecase,
    limbs_mul_low_same_length_basecase_alt, limbs_mul_low_same_length_divide_and_conquer,
    limbs_mul_low_same_length_divide_and_conquer_shared_scratch,
};
#[cfg(not(feature = "32_bit_limbs"))]
use malachite_nz::natural::arithmetic::mul::mul_mod::limbs_mul_mod_base_pow_n_minus_1;
use malachite_nz::natural::arithmetic::mul::product_of_limbs::limbs_product;
use malachite_nz::natural::arithmetic::mul::toom::{
    limbs_mul_greater_to_out_toom_22, limbs_mul_greater_to_out_toom_22_scratch_len,
    limbs_mul_greater_to_out_toom_32, limbs_mul_greater_to_out_toom_32_scratch_len,
    limbs_mul_greater_to_out_toom_33, limbs_mul_greater_to_out_toom_33_scratch_len,
    limbs_mul_greater_to_out_toom_42, limbs_mul_greater_to_out_toom_42_scratch_len,
    limbs_mul_greater_to_out_toom_43, limbs_mul_greater_to_out_toom_43_scratch_len,
    limbs_mul_greater_to_out_toom_44, limbs_mul_greater_to_out_toom_44_scratch_len,
    limbs_mul_greater_to_out_toom_52, limbs_mul_greater_to_out_toom_52_scratch_len,
    limbs_mul_greater_to_out_toom_53, limbs_mul_greater_to_out_toom_53_scratch_len,
    limbs_mul_greater_to_out_toom_54, limbs_mul_greater_to_out_toom_54_scratch_len,
    limbs_mul_greater_to_out_toom_62, limbs_mul_greater_to_out_toom_62_scratch_len,
    limbs_mul_greater_to_out_toom_63, limbs_mul_greater_to_out_toom_63_scratch_len,
    limbs_mul_greater_to_out_toom_6h, limbs_mul_greater_to_out_toom_6h_scratch_len,
    limbs_mul_greater_to_out_toom_8h, limbs_mul_greater_to_out_toom_8h_scratch_len,
};
use malachite_nz::natural::arithmetic::mul::{
    limbs_mul, limbs_mul_greater, limbs_mul_greater_to_out, limbs_mul_greater_to_out_basecase,
    limbs_mul_greater_to_out_scratch_len, limbs_mul_same_length_to_out,
    limbs_mul_same_length_to_out_scratch_len, limbs_mul_to_out, limbs_mul_to_out_scratch_len,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::{DoubleLimb, Limb};
use malachite_nz::test_util::generators::{
    natural_gen, natural_pair_gen, natural_triple_gen, natural_vec_gen,
    unsigned_vec_pair_gen_var_33, unsigned_vec_triple_gen_var_10, unsigned_vec_triple_gen_var_11,
    unsigned_vec_triple_gen_var_12, unsigned_vec_triple_gen_var_13, unsigned_vec_triple_gen_var_14,
    unsigned_vec_triple_gen_var_15, unsigned_vec_triple_gen_var_16, unsigned_vec_triple_gen_var_4,
    unsigned_vec_triple_gen_var_5, unsigned_vec_triple_gen_var_6, unsigned_vec_triple_gen_var_60,
    unsigned_vec_triple_gen_var_7, unsigned_vec_triple_gen_var_8, unsigned_vec_triple_gen_var_9,
};
use malachite_nz::test_util::natural::arithmetic::mul::natural_product_naive;
use malachite_nz::test_util::natural::arithmetic::mul::{
    limbs_mul_greater_to_out_basecase_mem_opt, limbs_product_naive,
};
use num::BigUint;
use rug;
use std::iter::{once, Product};
use std::str::FromStr;

fn series(start: Limb, len: usize) -> Vec<Limb> {
    (start..start + Limb::exact_from(len)).collect()
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mul_limb_and_limbs_vec_mul_limb_in_place() {
    let test = |limbs: &[Limb], limb: Limb, out: &[Limb]| {
        assert_eq!(limbs_mul_limb(limbs, limb), out);

        let mut limbs = limbs.to_vec();
        limbs_vec_mul_limb_in_place(&mut limbs, limb);
        assert_eq!(limbs, out);
    };
    test(&[], 0, &[]);
    test(&[], 5, &[]);
    test(&[6, 7], 2, &[12, 14]);
    test(&[100, 101, 102], 10, &[1000, 1010, 1020]);
    test(&[123, 456], 789, &[97047, 359784]);
    test(&[u32::MAX, 5], 2, &[u32::MAX - 1, 11]);
    test(&[u32::MAX], 2, &[u32::MAX - 1, 1]);
    test(&[u32::MAX], u32::MAX, &[1, u32::MAX - 1]);
}

#[test]
fn test_product() {
    let test = |xs, out| {
        let xs = vec_from_str(xs).unwrap();
        let product = Natural::product(xs.iter().cloned());
        assert!(product.is_valid());
        assert_eq!(product.to_string(), out);

        let product_alt = Natural::product(xs.iter());
        assert!(product_alt.is_valid());
        assert_eq!(product_alt, product);

        let product_alt = natural_product_naive(xs.into_iter());
        assert!(product_alt.is_valid());
        assert_eq!(product_alt, product);
    };
    test("[]", "1");
    test("[10]", "10");
    test("[6, 2]", "12");
    test("[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]", "0");
    test("[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]", "3628800");
    test(
        "[123456, 789012, 345678, 9012345]",
        "303462729062737285547520",
    );
}

#[test]
fn limbs_mul_limb_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen().test_properties_with_config(&config, |(xs, y)| {
        assert_eq!(
            Natural::from_owned_limbs_asc(limbs_mul_limb(&xs, y)),
            Natural::from_limbs_asc(&xs) * Natural::from(y)
        );
    });
}

#[test]
fn limbs_vec_mul_limb_in_place_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen().test_properties_with_config(&config, |(mut xs, y)| {
        let old_xs = xs.clone();
        limbs_vec_mul_limb_in_place(&mut xs, y);
        let n = Natural::from_limbs_asc(&old_xs) * Natural::from(y);
        assert_eq!(Natural::from_owned_limbs_asc(xs), n);
    });
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mul_limb_with_carry_to_out() {
    let test = |out_before: &[Limb],
                xs: &[Limb],
                y: Limb,
                carry: Limb,
                carry_out: Limb,
                out_after: &[Limb]| {
        let mut out = out_before.to_vec();
        assert_eq!(
            limbs_mul_limb_with_carry_to_out(&mut out, xs, y, carry),
            carry_out
        );
        assert_eq!(out, out_after);
    };
    test(
        &[10, 10, 10, 10],
        &[123, 456],
        789,
        10,
        0,
        &[97057, 359784, 10, 10],
    );
    test(&[10, 10, 10, 10], &[u32::MAX], 2, 3, 2, &[1, 10, 10, 10]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_limb_with_carry_to_out_fail() {
    limbs_mul_limb_with_carry_to_out(&mut [10], &[10, 10], 10, 2);
}

#[test]
fn limbs_mul_limb_with_carry_to_out_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    large_type_gen_var_1().test_properties_with_config(&config, |(mut out, xs, y, carry)| {
        let old_out = out.clone();
        let carry_out = limbs_mul_limb_with_carry_to_out(&mut out, &xs, y, carry);
        let len = xs.len();
        let n = Natural::from_owned_limbs_asc(xs) * Natural::from(y) + Natural::from(carry);
        let mut xs_out = n.into_limbs_asc();
        assert_eq!(carry_out != 0, xs_out.len() == len + 1);
        if carry_out != 0 {
            assert_eq!(*xs_out.last().unwrap(), carry_out);
        }
        xs_out.resize(len, 0);
        assert_eq!(xs_out, &out[..len]);
        assert_eq!(&out[len..], &old_out[len..]);
    });
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mul_limb_to_out() {
    let test = |out_before: &[Limb], xs: &[Limb], y: Limb, carry: Limb, out_after: &[Limb]| {
        let mut out = out_before.to_vec();
        assert_eq!(limbs_mul_limb_to_out(&mut out, xs, y), carry);
        assert_eq!(out, out_after);
    };
    test(&[10, 10, 10, 10], &[], 0, 0, &[10, 10, 10, 10]);
    test(&[10, 10, 10, 10], &[], 5, 0, &[10, 10, 10, 10]);
    test(&[10, 10, 10, 10], &[6, 7], 2, 0, &[12, 14, 10, 10]);
    test(
        &[10, 10, 10, 10],
        &[100, 101, 102],
        10,
        0,
        &[1000, 1010, 1020, 10],
    );
    test(
        &[10, 10, 10, 10],
        &[123, 456],
        789,
        0,
        &[97047, 359784, 10, 10],
    );
    test(
        &[10, 10, 10, 10],
        &[u32::MAX, 5],
        2,
        0,
        &[u32::MAX - 1, 11, 10, 10],
    );
    test(
        &[10, 10, 10, 10],
        &[u32::MAX],
        2,
        1,
        &[u32::MAX - 1, 10, 10, 10],
    );
    test(
        &[10, 10, 10, 10],
        &[u32::MAX],
        u32::MAX,
        u32::MAX - 1,
        &[1, 10, 10, 10],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_limb_to_out_fail() {
    limbs_mul_limb_to_out(&mut [10], &[10, 10], 10);
}

#[test]
fn limbs_mul_limb_to_out_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_1().test_properties_with_config(
        &config,
        |(mut out, xs, y)| {
            let old_out = out.clone();
            let carry = limbs_mul_limb_to_out(&mut out, &xs, y);
            let len = xs.len();
            let n = Natural::from_owned_limbs_asc(xs) * Natural::from(y);
            let mut limbs = n.into_limbs_asc();
            assert_eq!(carry != 0, limbs.len() == len + 1);
            if carry != 0 {
                assert_eq!(*limbs.last().unwrap(), carry);
            }
            limbs.resize(len, 0);
            assert_eq!(limbs, &out[..len]);
            assert_eq!(&out[len..], &old_out[len..]);
        },
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mul_limb_with_carry_in_place() {
    let test = |xs_before: &[Limb], y: Limb, carry: Limb, carry_out: Limb, xs_after: &[Limb]| {
        let mut xs = xs_before.to_vec();
        assert_eq!(
            limbs_slice_mul_limb_with_carry_in_place(&mut xs, y, carry),
            carry_out
        );
        assert_eq!(xs, xs_after);
    };
    test(&[123, 456], 789, 10, 0, &[97057, 359784]);
    test(&[u32::MAX], 2, 3, 2, &[1]);
}

#[test]
fn limbs_slice_mul_limb_with_carry_in_place_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_unsigned_triple_gen().test_properties_with_config(
        &config,
        |(mut xs, y, carry)| {
            let n = Natural::from_limbs_asc(&xs) * Natural::from(y) + Natural::from(carry);
            let carry_out = limbs_slice_mul_limb_with_carry_in_place(&mut xs, y, carry);
            let mut expected_limbs = n.into_limbs_asc();
            assert_eq!(carry_out != 0, expected_limbs.len() == xs.len() + 1);
            if carry_out != 0 {
                assert_eq!(*expected_limbs.last().unwrap(), carry_out);
            }
            expected_limbs.resize(xs.len(), 0);
            assert_eq!(xs, expected_limbs);
        },
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_slice_mul_limb_in_place() {
    let test = |xs: &[Limb], y: Limb, carry: Limb, out: &[Limb]| {
        let mut xs = xs.to_vec();
        assert_eq!(limbs_slice_mul_limb_in_place(&mut xs, y), carry);
        assert_eq!(xs, out);
    };
    test(&[], 0, 0, &[]);
    test(&[], 5, 0, &[]);
    test(&[6, 7], 2, 0, &[12, 14]);
    test(&[100, 101, 102], 10, 0, &[1000, 1010, 1020]);
    test(&[123, 456], 789, 0, &[97047, 359784]);
    test(&[u32::MAX, 5], 2, 0, &[u32::MAX - 1, 11]);
    test(&[u32::MAX], 2, 1, &[u32::MAX - 1]);
    test(&[u32::MAX], u32::MAX, u32::MAX - 1, &[1]);
}

#[test]
fn limbs_slice_mul_limb_in_place_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen().test_properties_with_config(&config, |(mut xs, y)| {
        let old_xs = xs.clone();
        let carry = limbs_slice_mul_limb_in_place(&mut xs, y);
        let n = Natural::from_limbs_asc(&old_xs) * Natural::from(y);
        let mut expected_limbs = n.into_limbs_asc();
        assert_eq!(carry != 0, expected_limbs.len() == xs.len() + 1);
        if carry != 0 {
            assert_eq!(*expected_limbs.last().unwrap(), carry);
        }
        expected_limbs.resize(xs.len(), 0);
        assert_eq!(xs, expected_limbs);
    });
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mul_greater() {
    let test = |xs: Vec<Limb>, ys: Vec<Limb>, result: Vec<Limb>| {
        assert_eq!(limbs_mul_greater(&xs, &ys), result);
    };
    test(vec![2], vec![3], vec![6, 0]);
    test(vec![1; 3], series(1, 3), vec![1, 3, 6, 5, 3, 0]);
    test(series(1, 3), vec![6, 7], vec![6, 19, 32, 21, 0]);
    test(
        vec![100, 101, 102],
        vec![102, 101, 100],
        vec![10200, 20402, 30605, 20402, 10200, 0],
    );
    test(vec![u32::MAX], vec![1], vec![u32::MAX, 0]);
    test(vec![u32::MAX], vec![u32::MAX], vec![1, u32::MAX - 1]);
    test(
        vec![u32::MAX; 3],
        vec![u32::MAX; 3],
        vec![1, 0, 0, u32::MAX - 1, u32::MAX, u32::MAX],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn test_limbs_mul_greater_fail_1() {
    limbs_mul_greater(&[6, 7], &[1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn test_limbs_mul_greater_fail_2() {
    limbs_mul_greater(&[6, 7], &[]);
}

fn limbs_mul_helper(f: &dyn Fn(&[Limb], &[Limb]) -> Vec<Limb>, xs: Vec<Limb>, ys: Vec<Limb>) {
    let result = f(&xs, &ys);
    assert_eq!(result.len(), xs.len() + ys.len());
    assert_eq!(
        Natural::from_owned_limbs_asc(result),
        Natural::from_owned_limbs_asc(xs) * Natural::from_owned_limbs_asc(ys)
    );
}

#[test]
fn limbs_mul_greater_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen_var_1().test_properties_with_config(&config, |(xs, ys)| {
        limbs_mul_helper(&limbs_mul_greater, xs, ys);
    });
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mul() {
    let test = |xs: Vec<Limb>, ys: Vec<Limb>, result: Vec<Limb>| {
        assert_eq!(limbs_mul(&xs, &ys), result);
    };
    test(vec![2], vec![3], vec![6, 0]);
    test(vec![1; 3], series(1, 3), vec![1, 3, 6, 5, 3, 0]);
    test(series(1, 3), vec![6, 7], vec![6, 19, 32, 21, 0]);
    test(vec![6, 7], series(1, 3), vec![6, 19, 32, 21, 0]);
    test(
        vec![100, 101, 102],
        vec![102, 101, 100],
        vec![10200, 20402, 30605, 20402, 10200, 0],
    );
    test(vec![u32::MAX], vec![1], vec![u32::MAX, 0]);
    test(vec![u32::MAX], vec![u32::MAX], vec![1, u32::MAX - 1]);
    test(
        vec![u32::MAX; 3],
        vec![u32::MAX; 3],
        vec![1, 0, 0, u32::MAX - 1, u32::MAX, u32::MAX],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn test_limbs_mul_fail() {
    limbs_mul(&[6, 7], &[]);
}

#[test]
fn limbs_mul_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen_var_2().test_properties_with_config(&config, |(xs, ys)| {
        limbs_mul_helper(&limbs_mul, xs, ys);
    });
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mul_same_length_to_out() {
    let test = |xs: Vec<Limb>, ys: Vec<Limb>, out_before: Vec<Limb>, out_after| {
        let mut out = out_before;
        let mut mul_scratch = vec![0; limbs_mul_same_length_to_out_scratch_len(xs.len())];
        limbs_mul_same_length_to_out(&mut out, &xs, &ys, &mut mul_scratch);
        assert_eq!(out, out_after);
    };
    test(vec![2], vec![3], vec![10; 3], vec![6, 0, 10]);
    test(
        vec![1; 3],
        series(1, 3),
        vec![5; 8],
        vec![1, 3, 6, 5, 3, 0, 5, 5],
    );
    test(
        vec![100, 101, 102],
        vec![102, 101, 100],
        vec![10; 7],
        vec![10200, 20402, 30605, 20402, 10200, 0, 10],
    );
    test(vec![u32::MAX], vec![1], vec![10; 3], vec![u32::MAX, 0, 10]);
    test(
        vec![u32::MAX],
        vec![u32::MAX],
        vec![10; 4],
        vec![1, u32::MAX - 1, 10, 10],
    );
    test(
        vec![u32::MAX; 3],
        vec![u32::MAX; 3],
        vec![10; 6],
        vec![1, 0, 0, u32::MAX - 1, u32::MAX, u32::MAX],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_same_length_to_out_fail_1() {
    let mut out = vec![10, 10, 10, 10, 10];
    let mut mul_scratch = vec![0; limbs_mul_same_length_to_out_scratch_len(3)];
    limbs_mul_same_length_to_out(&mut out, &[6, 7], &[1, 2, 3], &mut mul_scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_same_length_to_out_fail_2() {
    let mut out = vec![10, 10, 10];
    let mut mul_scratch = vec![0; limbs_mul_same_length_to_out_scratch_len(2)];
    limbs_mul_same_length_to_out(&mut out, &[6, 7], &[1, 2], &mut mul_scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_same_length_to_out_fail_3() {
    let mut out = vec![10];
    let mut mul_scratch = vec![0; limbs_mul_same_length_to_out_scratch_len(1)];
    limbs_mul_same_length_to_out(&mut out, &[], &[], &mut mul_scratch);
}

#[test]
fn limbs_mul_same_length_to_out_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_triple_gen_var_1().test_properties_with_config(&config, |(mut out, xs, ys)| {
        let old_out = out.clone();
        let mut mul_scratch = vec![0; limbs_mul_same_length_to_out_scratch_len(xs.len())];
        limbs_mul_same_length_to_out(&mut out, &xs, &ys, &mut mul_scratch);
        let len = xs.len() << 1;
        let n = Natural::from_owned_limbs_asc(xs) * Natural::from_owned_limbs_asc(ys);
        let mut expected_out = n.into_limbs_asc();
        expected_out.resize(len, 0);
        assert_eq!(expected_out, &out[..len]);
        assert_eq!(&out[len..], &old_out[len..]);
    });
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mul_greater_to_out() {
    let test =
        |xs: Vec<Limb>, ys: Vec<Limb>, out_before: Vec<Limb>, highest_result_limb, out_after| {
            let mut out = out_before.clone();
            limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys);
            assert_eq!(out, out_after);
            let mut out = out_before;
            let mut mul_scratch = vec![0; limbs_mul_greater_to_out_scratch_len(xs.len(), ys.len())];
            assert_eq!(
                limbs_mul_greater_to_out(&mut out, &xs, &ys, &mut mul_scratch),
                highest_result_limb
            );
            assert_eq!(out, out_after);
        };
    test(vec![2], vec![3], vec![10; 3], 0, vec![6, 0, 10]);
    test(
        vec![1; 3],
        series(1, 3),
        vec![5; 8],
        0,
        vec![1, 3, 6, 5, 3, 0, 5, 5],
    );
    test(
        series(1, 3),
        vec![6, 7],
        vec![0; 5],
        0,
        vec![6, 19, 32, 21, 0],
    );
    test(
        vec![100, 101, 102],
        vec![102, 101, 100],
        vec![10; 7],
        0,
        vec![10200, 20402, 30605, 20402, 10200, 0, 10],
    );
    test(
        vec![u32::MAX],
        vec![1],
        vec![10; 3],
        0,
        vec![u32::MAX, 0, 10],
    );
    test(
        vec![u32::MAX],
        vec![u32::MAX],
        vec![10; 4],
        u32::MAX - 1,
        vec![1, u32::MAX - 1, 10, 10],
    );
    test(
        vec![u32::MAX; 3],
        vec![u32::MAX; 3],
        vec![10; 6],
        u32::MAX,
        vec![1, 0, 0, u32::MAX - 1, u32::MAX, u32::MAX],
    );
}

#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_limbs_mul_greater_to_out() {
    let test = |xs: Vec<Limb>,
                ys: Vec<Limb>,
                out_before: Vec<Limb>,
                highest_result_limb,
                out_after: Vec<Limb>| {
        let mut out = out_before.clone();
        limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys);
        assert_eq!(out, out_after);
        let mut out = out_before;
        let mut mul_scratch = vec![0; limbs_mul_greater_to_out_scratch_len(xs.len(), ys.len())];
        assert_eq!(
            limbs_mul_greater_to_out(&mut out, &xs, &ys, &mut mul_scratch),
            highest_result_limb
        );
        assert_eq!(out, out_after);
    };
    test(
        vec![
            12709525342598979476,
            11509224300783138838,
            393964466388471327,
            14232928317295888119,
            13732076968605655238,
            15707747516397285107,
            8732499155930007762,
            9865419549041312495,
            1072105962419307309,
            17879862180104468293,
            17068561868445402638,
            1866096115219256579,
            6855809432664356654,
            1393588600207482081,
            3815615263372249309,
            4991042746319579278,
            15465001638246553057,
            16087255091514657862,
            5044736888141764583,
            2779747905405388451,
            583627419881315049,
            4335440233116895431,
            9498366494409866085,
            8645413868323766569,
            10728932435460960803,
            2150088858744686298,
            9562258835198494013,
            17827191588813955307,
            13784645620229731318,
            6296382068077415813,
            11689157886369173742,
            17754283123435968230,
            1461090916898201642,
            1288610558427232175,
            8104103888086828127,
            8032176301470720968,
            12200951204181999648,
            9485404571555436368,
            10222400990562648891,
            7599079551870785840,
            11203522977648266038,
            18338050175108207759,
            12077517359207243953,
            1611826255828631363,
            15837930674386751191,
            461582836356148934,
            16860704548252932358,
            6041830073720446893,
            16833467135075592988,
            17496069201595551026,
            10318992522772404757,
            13524041658187206186,
            4506190253313254923,
            17231310439197044711,
            13546843641632582646,
            7651927160556964586,
            9190936365032409113,
            9913829309881494193,
            12083378901909297431,
            287809474670991447,
            7728504523588277111,
            16741959317437359789,
            9529628049862654500,
            13419553383911388725,
            13376506161484177119,
            8176094358898406545,
            13190641351956097350,
            12568817836646213483,
            3486855157615498387,
            14702408831781542831,
            98979558380456360,
            13178740567667926424,
            17968226918253726036,
            9245411514261104062,
            2907160002341084957,
            1980943369383764848,
            6564658997896463013,
            13002525217264158147,
            444335199954317567,
            16863109666887451301,
            11108312108618761726,
            7630498129836691848,
            5435171385264263429,
            10900954800466810672,
            16615568172072435554,
            4587778094537619469,
            10961411668824294491,
            17076006711925106918,
            5351453460500307468,
            7629258051101663482,
            16987233075558685907,
            7043270533100406405,
            1951394976862135309,
            13689790855612457174,
            2278410054587042806,
            17001418508549255568,
            11660613606297735566,
            2855471124776808043,
            10065175989746288214,
            5055380914672426055,
            14126789454240780537,
            12269074817478677711,
            15318371572624008687,
            17454608042943339898,
            266175370449515926,
            10258065246409153340,
            7913204199005555677,
            2015162987422567864,
            947599677954716876,
            4742046749175767719,
            4638837750457895566,
            7651567606901939182,
            1517253577417508168,
            10046266355925262869,
            17517100300192594127,
            1840911236085514585,
            17925925769498956584,
            649534223875437606,
            4629497761671206138,
            6940070820665949971,
            11499097596331562367,
            6066662975772729994,
            14012285030054181,
            15069455665359536649,
            15693110947967738351,
            6137496527357766025,
            17475548660925055392,
            7927767715587303464,
            8531250928287116369,
            11631019584025111086,
            14052618820385867457,
            3118784937861555333,
            183376537996253187,
            5402208906954882112,
            4415682544496910978,
            4679985880085587510,
            8446593383458511654,
            15604884470214334395,
            1138627504082666786,
            10518517338133402469,
            4864527599992532396,
            13936709675277621349,
            564858321719103528,
            17954444276502582707,
            7973580172007214272,
            5448002835725952206,
            17643582902922581336,
            6555152033284063414,
            10046934352485272915,
            13709275446788968044,
            3781650587286942277,
            856963687705102140,
            14231130335064067998,
            12012182879109532509,
            14408852620208381144,
            18399276894609882283,
            8786560043544514971,
            9040897797784271497,
            3289492106827693022,
            2537349882995177,
            9547621122715901978,
            18031028021813298205,
            13759002194232082515,
            8230796682226694274,
            16497307259652601605,
            4532459299949788115,
            9096244343722808065,
            15835940740299471650,
            16962714874349511272,
            12751247365216834654,
            5665483802634541323,
            5932524403187765899,
            9724384992325030819,
            4567260165559481416,
            12988617208150638305,
            3810267272444218110,
            12215376695703433417,
            15917770368925117452,
            10076439022504697381,
            15419145301042845244,
            12520526582226591499,
            10731090420480517823,
            7095369748974936538,
            7265727720840614264,
            14328612713128953576,
            9796482770007505116,
            12823579117053411854,
            5055821064893108365,
            12630218511428402185,
            5189066330700437332,
            5109762584664698804,
            10723647767885063411,
            15665488082047274789,
            4569259999401990745,
            3735809282876517401,
            7285645596632892122,
            10962161595160119179,
            4816357354602339409,
            10873333778222769846,
            8013894684430460951,
            1877066355845544058,
            9184544411342906577,
            1919785302854216344,
            11743122258369849152,
            3429022440866576828,
            7006536176872515230,
            12354403703503061654,
            16589390678996741825,
            8567087982659180500,
            8236943971468579104,
            14606686909208062275,
            4461894282559697271,
            2623848805529273446,
            6820927267034378486,
            14393599619569715182,
            5199894747923716756,
            206321364421131251,
            4071363758717862161,
            11965914866997389305,
            8109905844769656284,
            3492539957359367736,
            16856371139562646083,
            4333218987558602531,
            16089995180748288161,
            11191427142864224026,
            2622884336931596940,
            16105201810330583174,
            11381683384063743452,
            7873797126730706201,
            11178731223505120377,
            16800978016652635785,
            13461685021093913844,
            3055116765865974959,
            3602813646007838280,
            829147354810404950,
            4780507430409308744,
            18314860632940156926,
            6995096023652594133,
            5759566466885931830,
            4840076000542784388,
        ],
        vec![
            15245088662193948010,
            854969528224537163,
            192457876290468361,
            3156774054099849881,
            10102117358735393641,
            13923135497401538045,
            15603007686998930972,
            3707765480829539463,
            1075990372015045994,
            4440028045035707188,
            779932550205535682,
            13284596850012603887,
            13447370325749987403,
            10657005451799608034,
            17344058779081327933,
            1801131630646010099,
            17879455113972297046,
            1049662270419803525,
            17887003202529550415,
            13730724178286439296,
            3086493866184691051,
            7455503161286080904,
            14945249663072669446,
            7413071270018261565,
            8165098975144402988,
            15667870805615006559,
            4534237642686726425,
            5675059133984408369,
            13542693529471369730,
            4650690134857994243,
            10593876026982724440,
            8719234160809710444,
            7340192483727047710,
            2225660849988538666,
            3260628781823840386,
            14784063213821786553,
            13478324037708856111,
            6239844587086244103,
            14508626048519473050,
            11443816492520902359,
            7084448144752764341,
            11673478635762496725,
            13444020463604694513,
            1798574113181758005,
            15195278749704748030,
            3490272214933312037,
            15632500462832370824,
            9808665338648603851,
            6377980234800091876,
            11306384233660763805,
            6392788317448223882,
            8005181869701567455,
            4601526777105113530,
            9348184476999479133,
            16105441815997897842,
            15373735633778437011,
            11733794529384137433,
            769246272107807645,
            2922899274256775805,
            16218486247871807873,
            10650657974127272786,
            579665301817927565,
            6403006378940431337,
            10150254532952843560,
            3736822004545760197,
            10244207440138560761,
            16631379436671010056,
            17418302422321190629,
            4844439457855539440,
            9662799133272397874,
            11622100630061039998,
            11017257064923257696,
            14025546287952884200,
            1170766120552674008,
            4852413824670160293,
            18019298735978800767,
            14042374992041286164,
            6103187929964524269,
            5988592592688695870,
            5579172720281387479,
            10738878044274955012,
            8401646271610146442,
            12016061916593958227,
            14752402557741497038,
            5053283107906893264,
            12910662726197463795,
            787526459034857809,
            10304827788120361107,
            8387521101013404665,
            6030209567663971422,
            7511028869236306454,
            11105170944119024313,
            2911699195421772292,
            11710398806568443147,
            7599646386487625804,
            2146501359265516686,
            1193294087739295886,
            16419769173966961854,
            14779980297792837632,
            6286361066120350249,
            8246126699673376536,
            2339493649448723726,
            12383521129608538925,
            17459816050942292574,
            7213741082075285427,
            14702683527305456088,
            17849030573001874153,
            3273901152373442943,
            10086273715179643444,
            14351251935054659627,
            3067622597087477151,
            4241957707372911307,
            16686513037697490920,
            1503886102490162470,
            4222986769290077389,
            17209928444872897872,
            10064374817012298812,
            1391022681726221923,
            3482099619102309134,
            151151415131464647,
            5477310851692317777,
            8185741896741403527,
            12297179519749775078,
            6980896315258250234,
            5491311995173541969,
            10908311176531272611,
            15140263006374103771,
            16292302828281485620,
            13488663273854028028,
            17078235461511918753,
            523009743565281503,
            11105648925812514991,
            13827146014280242829,
        ],
        vec![10; 373],
        3627981030815073084,
        vec![
            10242139703917377352,
            6869501223013262871,
            3374240433299030218,
            2448664517749959925,
            12614665088252879609,
            15275142410865499832,
            7415514779145416012,
            4634939093621563784,
            14236482271744498259,
            16987935748141823121,
            9662195261206294164,
            8327530275898959224,
            17948858401312480900,
            16590992031072707948,
            8981557837131782478,
            17292811815398261598,
            10343772151713015660,
            16403800522193054061,
            11001578312297934300,
            9055563331722809276,
            6861684031441187837,
            6179379396114830115,
            15119399843907730738,
            2747263417069121706,
            12218921993633141137,
            15314449116975726182,
            2870623933631129133,
            8433855307245599470,
            9663369547205952712,
            16656478013118492468,
            5014296474163082063,
            15045852603430413673,
            9039056709562337243,
            1775730247666171519,
            10284009922885822735,
            17264560580274867574,
            8659184543116501827,
            9757501208876960807,
            8107556209186377816,
            13981920571894003246,
            8697727621393221097,
            10111281749746035823,
            11547580803378751509,
            9866091897888262831,
            6932444770006022090,
            12886358055723945255,
            5304800874292271788,
            11523612872653318479,
            3158091624397075356,
            17376725070179056084,
            11353224313201827872,
            17568378701446022201,
            7083677611886335059,
            16630882804589833859,
            17071293908015663903,
            3335332823966517520,
            4551571010812166323,
            13956655949392024934,
            11137278406051829526,
            4896962372622758702,
            11813643533502615793,
            4581901117239147425,
            14593775693535083700,
            1510447605784811068,
            5229627540215802358,
            627282762635294446,
            4791499629805752429,
            10361188936025453248,
            4774699819640096953,
            13333081235316911046,
            10942108924892665866,
            16109048665458409419,
            15314689190690266823,
            7310144211471143550,
            16609282512397289062,
            4726986969285156208,
            8190850143304789515,
            1542721709305293842,
            12747051062417360628,
            10431119479932549840,
            5201202511017795553,
            7741819653940246707,
            2687570019862487900,
            6488374897263692453,
            13582359095544274953,
            14484022084752061384,
            5515018660843652873,
            8092760845580227599,
            2766454334996797883,
            18180569531864904251,
            3565091373050863067,
            12322298177624181037,
            11958316134617603606,
            9878952926876342598,
            5302556749713428981,
            8825508234500799831,
            11910303768250068310,
            326451074713852933,
            396115137030276241,
            12871099826433440706,
            4304686608856751519,
            13291683117058685204,
            9284558513918029302,
            3718112862893900571,
            9149607979222078382,
            16527849990011073270,
            7727656976171348203,
            6573616922619656711,
            16034045732340296623,
            5589930567903701070,
            10683802387965352692,
            2587318764516570107,
            10209376077208389402,
            17079775160514374021,
            6530000943687323192,
            3718180221149684880,
            18189517521112254114,
            5384059937475335410,
            609609335008271023,
            3896142000481106293,
            16658219747219000806,
            13778288993082074923,
            5582595801212065024,
            14131451705219445462,
            359265879248692883,
            16936956071544174710,
            15873091614777053247,
            2434660658463974075,
            2592075329848798604,
            14194422636533088807,
            7167203043317150959,
            11315395284250045497,
            2099964792961604585,
            1566230315841355073,
            16092624371341405050,
            11961408413498591004,
            16535764519833385727,
            16358286323170046012,
            460344037919559599,
            14169011134767857411,
            14106023301858944314,
            9862229088119368783,
            3357504073133417614,
            1102923706048286862,
            13563108311952119833,
            2143046275198271992,
            3979966820061122245,
            9996599392497104038,
            18113381312649782606,
            2789577810558634539,
            2505893204182156006,
            5459782976231228406,
            14606983256382437526,
            17289593816561860577,
            18339393907946617355,
            2258995667690444422,
            8894197183418385086,
            18341894792090069275,
            11391274557331577482,
            1810514891585419128,
            7148289154353071487,
            2119113819713138874,
            10271861970912805029,
            7676899172550981679,
            8624446484952065684,
            5656835932522160843,
            16121420157925734770,
            14082469678115799904,
            10255722969024722343,
            1662291548355896945,
            6389399781869176377,
            12541949260215847692,
            7144384677827477175,
            7282727395448468407,
            12150328138548453752,
            3472826938551266818,
            3748717285404782518,
            4780951876470252398,
            16824789631565337027,
            15839657600800222362,
            10490421032004326835,
            13577644447718918237,
            6631866657761640573,
            981988710472148081,
            2178386723930299579,
            4509700998976576445,
            7085230111532242363,
            15742412314382227975,
            1728841640197493179,
            9413625884895979097,
            17011657766150783108,
            7453240289129863943,
            14356227040755802558,
            8727877790598607374,
            6442754498625288772,
            7392839418676330133,
            14897473376443001590,
            6366239299333724836,
            9325048183999746407,
            3440007856498006275,
            15045549040986890155,
            14490217742673747202,
            15278519326741232153,
            6554990600736077400,
            14130125167723787542,
            13808426692316226071,
            5353065033486249758,
            1150929840748552011,
            12174365818942611676,
            8325746398590604908,
            1113711086245528668,
            10314080011282631969,
            8859099728742225981,
            17921003815641223868,
            3105591894080053316,
            15297241643811159913,
            17457505649562044023,
            2057745826478669102,
            4884750341838598362,
            16356083315993976675,
            9101726946186468401,
            14354106624548323412,
            9370983965070459320,
            7738440263290367492,
            8917029115165607445,
            2195272365194297011,
            9558528105439905340,
            8030625988101552152,
            3397298078799745318,
            9736711397155048940,
            2460615496131263256,
            15140262761582421804,
            6475970420636983572,
            15248938774827950731,
            3601954052709238929,
            3331181884995306372,
            5102925218725345470,
            3667800919502192980,
            3004240798812901640,
            5533280375618915829,
            5107586811468410440,
            10224278806003351694,
            6476631719990351159,
            2003572440276248817,
            12145591924789065893,
            12519472837420433198,
            17782311381537397376,
            5661681639850380779,
            10364042884708771431,
            1695509166589750758,
            14618040013233272293,
            13897753646384009557,
            3750141258844328560,
            2521464661853413252,
            6278261092787958914,
            10406656955141036872,
            3275028812461691395,
            12925016960899871398,
            9841323801652199824,
            6731798363764447936,
            16237427233498849184,
            18347750732935739357,
            13607917507344538804,
            7384152260726915035,
            5463321849210106032,
            16371815515677214027,
            8832632406517327899,
            8268519393788774229,
            3669915584804849690,
            1269556794163966064,
            17809810670965530193,
            15840056410475484689,
            4076484974516366053,
            9606036112446697020,
            4868364087195523723,
            2058736710151928793,
            10897356652781847288,
            465078533977740804,
            16624721881706870960,
            11964802914009109220,
            16816447700891974174,
            3690088264896558634,
            7147174284749790878,
            9145833698882950169,
            8784425020769374260,
            1861407397398056953,
            12834090279159906525,
            7662444631348458158,
            14132542110004422269,
            9328868421834468934,
            16741858393921937122,
            15694875428320019398,
            14994603044491181705,
            9746169230120941506,
            7938113624909327958,
            13340368850853929072,
            7651461065837541114,
            4476063835934825867,
            8693815106327834329,
            4988903731011095573,
            4462304735549234904,
            8428813472483033040,
            4788353502021621362,
            12565779699104064716,
            3068625859269266553,
            2676497072041684077,
            6224517250248606913,
            10155182336700912649,
            4789285195609433162,
            16223567669480014548,
            7428311545244653529,
            16614898660347133660,
            18158662267123053326,
            3623664384860961629,
            7652479537264294496,
            8606705184150939035,
            8275137754514789786,
            1964775551555279291,
            1438900651657172959,
            17673106577623030111,
            6973507447558830112,
            15423010115647882749,
            3472057791767023426,
            5848284772473138216,
            2890665771216618892,
            9314934874321984084,
            8734072522843080836,
            14327114341827253389,
            4434569283869444664,
            12983438456823218137,
            10233784466676928341,
            16291964559077701283,
            9678367439362828938,
            7822571001351685129,
            15858728294074423230,
            10852593355325202465,
            12352889154252904310,
            12472638922147628465,
            858263613939813395,
            12316650043304516782,
            15905737765854758582,
            5901848627660760442,
            8757164316447879955,
            1327307879496998670,
            16419197412056777506,
            11241454192064071141,
            5669708276938693043,
            3575232442870910395,
            17428985550073658716,
            15774059386607531181,
            16583828728361725400,
            2690073375909258220,
            3913838352065921651,
            11930346452395449553,
            7508883471181453189,
            1979183109855731075,
            17205167019849574415,
            6202127415457809639,
            15616037631795263171,
            13758692529633453573,
            4703023550466089121,
            6440538290327762255,
            4617056828336967974,
            1016260815990065725,
            10542530738319933236,
            13839074645977905740,
            6298401080101494064,
            18179185785092266771,
            808757022663954675,
            655320984483308172,
            6492165610369841580,
            10622849106810507658,
            3627981030815073084,
        ],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_fail_1() {
    let mut out = vec![10; 4];
    let xs = series(6, 3);
    limbs_mul_greater_to_out_basecase(&mut out, &xs, &[1, 2]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_fail_2() {
    let mut out = vec![10; 5];
    let ys = series(1, 3);
    limbs_mul_greater_to_out_basecase(&mut out, &[6, 7], &ys);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_fail_3() {
    let mut out = vec![10; 3];
    limbs_mul_greater_to_out_basecase(&mut out, &[6, 7], &[]);
}

fn limbs_mul_basecase_helper(out: &[Limb], xs: &[Limb], ys: &[Limb]) -> Vec<Limb> {
    let mut out = out.to_vec();
    let old_out = out.clone();
    limbs_mul_greater_to_out_basecase(&mut out, xs, ys);
    let n = Natural::from_limbs_asc(xs) * Natural::from_limbs_asc(ys);
    let len = xs.len() + ys.len();
    let mut limbs = n.into_limbs_asc();
    limbs.resize(len, 0);
    assert_eq!(limbs, &out[..len]);
    assert_eq!(&out[len..], &old_out[len..]);
    out
}

#[test]
fn limbs_mul_greater_to_out_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 1024);
    config.insert("mean_stripe_n", 512 << Limb::LOG_WIDTH);
    unsigned_vec_triple_gen_var_2().test_properties_with_config(&config, |(mut out, xs, ys)| {
        let old_out = out.clone();
        let expected_out = limbs_mul_basecase_helper(&out, &xs, &ys);
        let mut mul_scratch = vec![0; limbs_mul_greater_to_out_scratch_len(xs.len(), ys.len())];
        let highest_result_limb = limbs_mul_greater_to_out(&mut out, &xs, &ys, &mut mul_scratch);
        assert_eq!(highest_result_limb, out[xs.len() + ys.len() - 1]);
        assert_eq!(out, expected_out);
        let mut out = old_out;
        limbs_mul_greater_to_out_basecase_mem_opt(&mut out, &xs, &ys);
        assert_eq!(out, expected_out);
    });
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mul_to_out() {
    let test = |xs: Vec<Limb>, ys: Vec<Limb>, out_before: Vec<Limb>, out_after| {
        let mut out = out_before;
        let mut mul_scratch = vec![0; limbs_mul_to_out_scratch_len(xs.len(), ys.len())];
        limbs_mul_to_out(&mut out, &xs, &ys, &mut mul_scratch);
        assert_eq!(out, out_after);
    };
    test(vec![2], vec![3], vec![10; 3], vec![6, 0, 10]);
    test(
        vec![1; 3],
        series(1, 3),
        vec![5; 8],
        vec![1, 3, 6, 5, 3, 0, 5, 5],
    );
    test(series(1, 3), vec![6, 7], vec![0; 5], vec![6, 19, 32, 21, 0]);
    test(vec![6, 7], series(1, 3), vec![0; 5], vec![6, 19, 32, 21, 0]);
    test(
        vec![100, 101, 102],
        vec![102, 101, 100],
        vec![10; 7],
        vec![10200, 20402, 30605, 20402, 10200, 0, 10],
    );
    test(vec![u32::MAX], vec![1], vec![10; 3], vec![u32::MAX, 0, 10]);
    test(
        vec![u32::MAX],
        vec![u32::MAX],
        vec![10; 4],
        vec![1, u32::MAX - 1, 10, 10],
    );
    test(
        vec![u32::MAX; 3],
        vec![u32::MAX; 3],
        vec![10; 6],
        vec![1, 0, 0, u32::MAX - 1, u32::MAX, u32::MAX],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_to_out_fail_1() {
    let mut out = vec![10, 10, 10];
    let mut mul_scratch = vec![0; limbs_mul_to_out_scratch_len(2, 2)];
    limbs_mul_to_out(&mut out, &[6, 7], &[1, 2], &mut mul_scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_to_out_fail_2() {
    let mut out = vec![10, 10, 10, 10];
    let mut mul_scratch = vec![0; limbs_mul_to_out_scratch_len(1, 3)];
    limbs_mul_to_out(&mut out, &[], &[1, 2, 3], &mut mul_scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_to_out_fail_3() {
    let mut out = vec![10, 10, 10, 10];
    let mut mul_scratch = vec![0; limbs_mul_to_out_scratch_len(1, 1)];
    limbs_mul_to_out(&mut out, &[1, 2, 3], &[], &mut mul_scratch);
}

#[test]
fn limbs_mul_to_out_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_triple_gen_var_3().test_properties_with_config(&config, |(mut out, xs, ys)| {
        let old_out = out.clone();
        let mut mul_scratch = vec![0; limbs_mul_to_out_scratch_len(xs.len(), ys.len())];
        let highest_result_limb = limbs_mul_to_out(&mut out, &xs, &ys, &mut mul_scratch);
        assert_eq!(highest_result_limb, out[xs.len() + ys.len() - 1]);
        let len = xs.len() + ys.len();
        let n = Natural::from_owned_limbs_asc(xs) * Natural::from_owned_limbs_asc(ys);
        let mut limbs = n.into_limbs_asc();
        limbs.resize(len, 0);
        assert_eq!(limbs, &out[..len]);
        assert_eq!(&out[len..], &old_out[len..]);
    });
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mul_greater_to_out_toom_22() {
    let test = |xs: &[Limb], ys: &[Limb], out_before: &[Limb], out_after: &[Limb]| {
        let mut scratch = vec![0; limbs_mul_greater_to_out_toom_22_scratch_len(xs.len(), ys.len())];
        let mut out = out_before.to_vec();
        limbs_mul_greater_to_out_basecase(&mut out, xs, ys);
        assert_eq!(out, out_after);
        let mut out = out_before.to_vec();
        limbs_mul_greater_to_out_toom_22(&mut out, xs, ys, &mut scratch);
        assert_eq!(out, out_after);
    };
    // - s != n
    // - !(xs0[s] == 0 && limbs_cmp_same_length(&xs0[..s], xs1) == Less)
    // - t != n
    // - slice_test_zero(&ys0[t..]) && limbs_cmp_same_length(&ys0[..t], ys1) != Less
    // - s <= t
    // - !v_neg_1_neg
    // - carry <= 2
    test(
        &series(2, 3),
        &series(3, 3),
        &[10; 6],
        &[6, 17, 34, 31, 20, 0],
    );
    // - xs0[s] == 0 && limbs_cmp_same_length(&xs0[..s], xs1) == Less
    // - v_neg_1_neg
    test(
        &[2, 0, 4],
        &[3, 4, 5],
        &[10, 10, 10, 10, 10, 10],
        &[6, 8, 22, 16, 20, 0],
    );
    test(&[1; 3], &series(1, 3), &[5; 8], &[1, 3, 6, 5, 3, 0, 5, 5]);
    // - s == n
    // - limbs_cmp_same_length(ys0, ys1) != Less
    // - t == n
    // - limbs_cmp_same_length(ys0, ys1) == Less
    test(&[1; 4], &series(1, 4), &[5; 8], &[1, 3, 6, 10, 9, 7, 4, 0]);
    // - limbs_cmp_same_length(&a0[..n], &a1[..n]) == Less
    // - limbs_cmp_same_length(&b0[..n], &b1[..n]) != Less
    test(&series(1, 4), &[1; 4], &[5; 8], &[1, 3, 6, 10, 9, 7, 4, 0]);
    // - slice_test_zero(&ys0[t..]) && limbs_cmp_same_length(&ys0[..t], ys1) == Less
    test(
        &series(1, 5),
        &[1, 0, 0, 4],
        &[5; 9],
        &[1, 2, 3, 8, 13, 12, 16, 20, 0],
    );
    // - s > t
    test(&[1; 4], &series(1, 3), &[5; 8], &[1, 3, 6, 6, 5, 3, 0, 5]);
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        &[10; 7],
        &[10200, 20402, 30605, 20402, 10200, 0, 10],
    );
    let xs = rle_decode(&[(u32::MAX, 8), (4294950911, 1), (u32::MAX, 21), (536870911, 1), (0, 25)]);
    let ys = rle_decode(&[(u32::MAX, 2), (4294963199, 1), (u32::MAX, 18), (268435455, 1), (0, 34)]);
    let out_len = xs.len() + ys.len();
    // - carry > 2
    test(
        &xs,
        &ys,
        &vec![10; out_len],
        &rle_decode(&[
            (1, 1),
            (0, 1),
            (4096, 1),
            (0, 5),
            (16384, 1),
            (0, 1),
            (67108864, 1),
            (0, 10),
            (4026531840, 1),
            (u32::MAX, 8),
            (3758095359, 1),
            (u32::MAX, 2),
            (4294966783, 1),
            (u32::MAX, 18),
            (33554431, 1),
            (0, 59),
        ]),
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_22_fail_1() {
    let mut scratch = vec![];
    let mut out = vec![10; 4];
    limbs_mul_greater_to_out_toom_22(&mut out, &[6], &[1], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_22_fail_2() {
    let mut scratch = vec![];
    let mut out = vec![10; 7];
    let xs = series(6, 3);
    let ys = series(1, 4);
    limbs_mul_greater_to_out_toom_22(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_22_fail_3() {
    let mut scratch = vec![];
    let mut out = vec![10; 7];
    let xs = series(6, 4);
    limbs_mul_greater_to_out_toom_22(&mut out, &xs, &[1, 2], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_22_fail_4() {
    let mut scratch = vec![];
    let mut out = vec![10; 7];
    let xs = series(6, 3);
    limbs_mul_greater_to_out_toom_22(&mut out, &xs, &[1, 2], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_22_fail_5() {
    let mut scratch = vec![];
    let mut out = vec![10; 4];
    let xs = series(6, 3);
    let ys = series(1, 3);
    limbs_mul_greater_to_out_toom_22(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_22_fail_6() {
    let mut scratch = vec![];
    let mut out = vec![10; 4];
    let xs = series(6, 3);
    limbs_mul_greater_to_out_toom_22(&mut out, &xs, &[], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_22_fail_7() {
    let mut scratch = vec![];
    let mut out = vec![10; 4];
    let xs = series(6, 3);
    let ys = series(1, 4);
    limbs_mul_greater_to_out_toom_22(&mut out, &xs, &ys, &mut scratch);
}

macro_rules! mul_properties_helper {
    ($properties: ident, $mul: ident, $scratch: ident, $gen: ident) => {
        #[test]
        fn $properties() {
            let mut config = GenConfig::new();
            config.insert("mean_length_n", 2048);
            config.insert("mean_stripe_n", 4 << Limb::LOG_WIDTH);
            $gen().test_properties_with_config(&config, |(mut out, xs, ys)| {
                let expected_out = limbs_mul_basecase_helper(&out, &xs, &ys);
                let mut scratch = vec![0; $scratch(xs.len(), ys.len())];
                $mul(&mut out, &xs, &ys, &mut scratch);
                assert_eq!(out, expected_out);
            });
        }
    };
}
mul_properties_helper!(
    limbs_mul_greater_to_out_toom_22_properties,
    limbs_mul_greater_to_out_toom_22,
    limbs_mul_greater_to_out_toom_22_scratch_len,
    unsigned_vec_triple_gen_var_4
);
mul_properties_helper!(
    limbs_mul_greater_to_out_toom_32_properties,
    limbs_mul_greater_to_out_toom_32,
    limbs_mul_greater_to_out_toom_32_scratch_len,
    unsigned_vec_triple_gen_var_5
);
mul_properties_helper!(
    limbs_mul_greater_to_out_toom_33_properties,
    limbs_mul_greater_to_out_toom_33,
    limbs_mul_greater_to_out_toom_33_scratch_len,
    unsigned_vec_triple_gen_var_6
);
mul_properties_helper!(
    limbs_mul_greater_to_out_toom_42_properties,
    limbs_mul_greater_to_out_toom_42,
    limbs_mul_greater_to_out_toom_42_scratch_len,
    unsigned_vec_triple_gen_var_7
);
mul_properties_helper!(
    limbs_mul_greater_to_out_toom_43_properties,
    limbs_mul_greater_to_out_toom_43,
    limbs_mul_greater_to_out_toom_43_scratch_len,
    unsigned_vec_triple_gen_var_8
);
mul_properties_helper!(
    limbs_mul_greater_to_out_toom_44_properties,
    limbs_mul_greater_to_out_toom_44,
    limbs_mul_greater_to_out_toom_44_scratch_len,
    unsigned_vec_triple_gen_var_9
);
mul_properties_helper!(
    limbs_mul_greater_to_out_toom_52_properties,
    limbs_mul_greater_to_out_toom_52,
    limbs_mul_greater_to_out_toom_52_scratch_len,
    unsigned_vec_triple_gen_var_10
);
mul_properties_helper!(
    limbs_mul_greater_to_out_toom_53_properties,
    limbs_mul_greater_to_out_toom_53,
    limbs_mul_greater_to_out_toom_53_scratch_len,
    unsigned_vec_triple_gen_var_11
);
mul_properties_helper!(
    limbs_mul_greater_to_out_toom_54_properties,
    limbs_mul_greater_to_out_toom_54,
    limbs_mul_greater_to_out_toom_54_scratch_len,
    unsigned_vec_triple_gen_var_12
);
mul_properties_helper!(
    limbs_mul_greater_to_out_toom_62_properties,
    limbs_mul_greater_to_out_toom_62,
    limbs_mul_greater_to_out_toom_62_scratch_len,
    unsigned_vec_triple_gen_var_13
);
mul_properties_helper!(
    limbs_mul_greater_to_out_toom_63_properties,
    limbs_mul_greater_to_out_toom_63,
    limbs_mul_greater_to_out_toom_63_scratch_len,
    unsigned_vec_triple_gen_var_14
);
mul_properties_helper!(
    limbs_mul_greater_to_out_toom_6h_properties,
    limbs_mul_greater_to_out_toom_6h,
    limbs_mul_greater_to_out_toom_6h_scratch_len,
    unsigned_vec_triple_gen_var_15
);
mul_properties_helper!(
    limbs_mul_greater_to_out_toom_8h_properties,
    limbs_mul_greater_to_out_toom_8h,
    limbs_mul_greater_to_out_toom_8h_scratch_len,
    unsigned_vec_triple_gen_var_16
);

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mul_greater_to_out_toom_32() {
    let test = |xs: Vec<Limb>, ys: Vec<Limb>, out_before: Vec<Limb>, out_after| {
        let mut out = out_before.to_vec();
        limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys);
        assert_eq!(out, out_after);
        let mut out = out_before.to_vec();
        let mut scratch = vec![0; limbs_mul_greater_to_out_toom_32_scratch_len(xs.len(), ys.len())];
        limbs_mul_greater_to_out_toom_32(&mut out, &xs, &ys, &mut scratch);
        assert_eq!(out, out_after);
    };
    // - !(ap1hi == 0 && limbs_cmp_same_length(ap1, xs1) == Less)
    // - t == n
    // - limbs_cmp_same_length(ys0, ys1) == Less
    // - ap1hi != 1 and ap1hi != 2
    // - !bp1hi
    // - hi == 0 first time
    // - v_neg_1_neg
    // - s <= t
    // - s + t > n
    // - hi >= 0 second time
    test(
        series(2, 6),
        series(3, 4),
        vec![10; 10],
        vec![6, 17, 34, 58, 76, 94, 88, 71, 42, 0],
    );
    // - limbs_cmp_same_length(ys0, ys1) != Less
    // - ap1hi == 2
    // - bp1hi
    // - !v_neg_1_neg
    test(
        vec![u32::MAX; 6],
        vec![u32::MAX; 4],
        vec![10; 10],
        vec![1, 0, 0, 0, u32::MAX, u32::MAX, u32::MAX - 1, u32::MAX, u32::MAX, u32::MAX],
    );
    // - ap1hi == 0 && limbs_cmp_same_length(ap1, xs1) == Less
    test(
        vec![0, 0, 1, 1, 0, 1],
        vec![0, 0, 0, 1],
        vec![10; 10],
        vec![0, 0, 0, 0, 0, 1, 1, 0, 1, 0],
    );
    // - t != n
    // - slice_test_zero(&ys0[t..]) && limbs_cmp_same_length(&ys0[..t], ys1) == Less
    // - s + t <= n
    test(
        vec![0, 0, 0, 0, 0, 0, 1],
        vec![0, 0, 0, 0, 1],
        vec![10; 12],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    );
    // - !(slice_test_zero(&ys0[t..]) && limbs_cmp_same_length(&ys0[..t], ys1) == Less)
    test(
        vec![0, 0, 0, 0, 0, 0, 1],
        vec![0, 0, 1, 0, 1],
        vec![10; 12],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0],
    );
    // - s > t
    test(
        series(1, 9),
        vec![9, 8, 7, 6, 5],
        vec![10; 14],
        vec![9, 26, 50, 80, 115, 150, 185, 220, 255, 200, 146, 94, 45, 0],
    );
    // - ap1hi == 1
    test(
        vec![2543705880, 1859419010, 3343322808, 1165039137, 1872701663, 1957510151, 1589243046],
        vec![1919189400, 1295801997, 354566481, 1212146910, 1886225431],
        vec![10; 14],
        vec![
            1753714240, 1114397484, 4100081063, 2352383720, 667557204, 920036609, 2291920497,
            3338154324, 3806846000, 1880963052, 291601955, 697949587, 10, 10,
        ],
    );
    // - hi != 0 first time
    test(
        vec![706760835, 4153647095, 3843998199, 2077172825, 1158686949, 3157624247],
        vec![2847735618, 2779635711, 2471732382, 2655639495],
        vec![10; 10],
        vec![
            2814066374, 2022835469, 2101335047, 312674723, 2952296274, 1055977952, 590716674,
            290888444, 3944399226, 1952404077,
        ],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_32_fail_1() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_32_scratch_len(1, 1)];
    let mut out = vec![10; 4];
    limbs_mul_greater_to_out_toom_32(&mut out, &[6], &[1], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_32_fail_2() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_32_scratch_len(3, 4)];
    let mut out = vec![10; 7];
    let xs = series(6, 3);
    let ys = series(1, 4);
    limbs_mul_greater_to_out_toom_32(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_32_fail_3() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_32_scratch_len(5, 4)];
    let mut out = vec![10; 9];
    let xs = series(6, 5);
    let ys = series(1, 4);
    limbs_mul_greater_to_out_toom_32(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_32_fail_4() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_32_scratch_len(6, 3)];
    let mut out = vec![10; 9];
    let xs = series(6, 6);
    let ys = series(1, 3);
    limbs_mul_greater_to_out_toom_32(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_32_fail_5() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_32_scratch_len(3, 0)];
    let mut out = vec![10; 4];
    let xs = series(6, 3);
    limbs_mul_greater_to_out_toom_32(&mut out, &xs, &[], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_32_fail_6() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_32_scratch_len(6, 4)];
    let mut out = vec![10; 9];
    let xs = series(6, 6);
    let ys = series(1, 4);
    limbs_mul_greater_to_out_toom_32(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mul_greater_to_out_toom_33() {
    let test = |xs: Vec<Limb>, ys: Vec<Limb>, out_before: Vec<Limb>, out_after| {
        let mut out = out_before.to_vec();
        limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys);
        assert_eq!(out, out_after);
        let mut out = out_before.to_vec();
        let mut scratch = vec![0; limbs_mul_greater_to_out_toom_33_scratch_len(xs.len(), ys.len())];
        limbs_mul_greater_to_out_toom_33(&mut out, &xs, &ys, &mut scratch);
        assert_eq!(out, out_after);
    };
    // - carry == 0 && limbs_cmp_same_length(&gp[..n], xs_1) == Less
    // - s != n
    // - carry == 0 && limbs_cmp_same_length(&gp[..n], ys_1) == Less
    // - t != n
    // - s <= t
    // - !v_neg_1
    // - two_r <= k + 1
    test(
        series(2, 5),
        series(3, 5),
        vec![10; 10],
        vec![6, 17, 34, 58, 90, 94, 88, 71, 42, 0],
    );
    // - s > t
    test(
        series(2, 6),
        series(3, 5),
        vec![10; 11],
        vec![6, 17, 34, 58, 90, 115, 116, 106, 84, 49, 0],
    );
    // - v_neg_1
    // - two_r > k + 1
    test(
        series(2, 9),
        series(3, 8),
        vec![10; 17],
        vec![6, 17, 34, 58, 90, 131, 182, 244, 296, 315, 320, 310, 284, 241, 180, 100, 0],
    );
    test(
        series(3, 5),
        series(2, 5),
        vec![10; 10],
        vec![6, 17, 34, 58, 90, 94, 88, 71, 42, 0],
    );
    // - !(carry == 0 && limbs_cmp_same_length(&gp[..n], xs_1) == Less)
    // - !(carry == 0 && limbs_cmp_same_length(&gp[..n], ys_1) == Less)
    test(
        vec![u32::MAX; 5],
        vec![u32::MAX; 5],
        vec![10; 10],
        vec![1, 0, 0, 0, 0, u32::MAX - 1, u32::MAX, u32::MAX, u32::MAX, u32::MAX],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_33_fail_1() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_33_scratch_len(1, 1)];
    let mut out = vec![10; 4];
    limbs_mul_greater_to_out_toom_33(&mut out, &[6], &[1], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_33_fail_2() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_33_scratch_len(5, 5)];
    let mut out = vec![10; 11];
    let xs = series(6, 5);
    let ys = series(1, 6);
    limbs_mul_greater_to_out_toom_33(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_33_fail_3() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_33_scratch_len(5, 5)];
    let mut out = vec![10; 9];
    let xs = series(6, 5);
    let ys = series(1, 4);
    limbs_mul_greater_to_out_toom_33(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_33_fail_4() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_33_scratch_len(5, 5)];
    let mut out = vec![10; 5];
    let xs = series(6, 5);
    limbs_mul_greater_to_out_toom_33(&mut out, &xs, &[], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_33_fail_5() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_33_scratch_len(6, 6)];
    let mut out = vec![10; 9];
    let xs = series(6, 6);
    let ys = series(1, 5);
    limbs_mul_greater_to_out_toom_33(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mul_greater_to_out_toom_42() {
    let test = |xs: Vec<Limb>, ys: Vec<Limb>, out_before: Vec<Limb>, out_after| {
        let mut out = out_before.to_vec();
        limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys);
        assert_eq!(out, out_after);
        let mut out = out_before.to_vec();
        let mut scratch = vec![0; limbs_mul_greater_to_out_toom_42_scratch_len(xs.len(), ys.len())];
        limbs_mul_greater_to_out_toom_42(&mut out, &xs, &ys, &mut scratch);
        assert_eq!(out, out_after);
    };
    // - v_neg_1_neg
    // - t == n
    // - limbs_cmp_same_length(ys_0, ys_1) == Less
    // - s <= t
    // - as1[n] not 1, 2, or 3
    test(
        series(2, 4),
        vec![3, 4],
        vec![10; 7],
        vec![6, 17, 24, 31, 20, 0, 10],
    );
    // - !v_neg_1_neg
    // - s != n
    // - t != n
    // - !(slice_test_zero(&ys_0[t..]) && limbs_cmp_same_length(&ys_0[..t], ys_1) == Less)
    test(
        series(2, 7),
        series(3, 3),
        vec![10; 10],
        vec![6, 17, 34, 46, 58, 70, 82, 67, 40, 0],
    );
    // - s > t
    test(
        series(2, 8),
        series(3, 3),
        vec![10; 11],
        vec![6, 17, 34, 46, 58, 70, 82, 94, 76, 45, 0],
    );
    // - limbs_cmp_same_length(ys_0, ys_1) != Less
    test(
        vec![0, 0, 0, 1],
        vec![1, 1],
        vec![10; 6],
        vec![0, 0, 0, 1, 1, 0],
    );
    // - as1[n] == 1
    test(
        vec![
            2363703565, 2011430902, 405935879, 3293866119, 79230945, 4067912411, 54522599,
            3863530924, 2648195217, 3696638907, 2693775185, 2466180916, 2288038816, 3085875921,
            2622914893, 3412444602, 1714899352, 1458044565, 4160795266,
        ],
        vec![
            2010684769, 395852000, 1475286147, 263729287, 1827966398, 926833006, 3647866695,
            2299638628,
        ],
        vec![10; 27],
        vec![
            2935529197, 2628679470, 2989406385, 4135607148, 3098618197, 1986483787, 2969118597,
            4064944337, 1353361316, 3300804798, 3539475248, 1813351909, 4189109323, 1508204245,
            3032195050, 2111172804, 2647234523, 763063403, 499753337, 484003129, 951290762,
            31889895, 4291170933, 743974460, 931456782, 3403938046, 2227799389,
        ],
    );
    // - bs1[n] != 0
    test(
        vec![
            1023706198, 1055957821, 62637438, 3129002448, 1343635842, 1979891039, 2332614953,
            820715064, 126240740, 3763174513, 874511155, 1433571832, 1799667271, 828081508,
            1790140791, 3456862168, 182082249,
        ],
        vec![
            272565221, 2271318511, 3915555663, 752672586, 2086228575, 93709012, 4089106295,
            1296382745, 4014782836, 4084383484,
        ],
        vec![10; 27],
        vec![
            2478924526, 600853546, 3764116188, 869876026, 49911338, 2430145334, 1531060628,
            4131353567, 2147110402, 1698823317, 3610138028, 2221603642, 2262453949, 2700908655,
            2085097953, 1179421079, 2314185794, 3274969801, 956808943, 183640877, 769743340,
            2499732116, 168215214, 1611459466, 1659741921, 3303732250, 173154690,
        ],
    );
    // - as1[n] == 2
    test(
        vec![
            0xfffff,
            0,
            0,
            4294965248,
            0x1ffffff,
            0,
            0,
            0,
            4294966784,
            u32::MAX,
            u32::MAX,
            u32::MAX,
            0,
            0x80000000,
            u32::MAX,
            u32::MAX,
            u32::MAX,
            u32::MAX,
            u32::MAX,
        ],
        vec![u32::MAX, u32::MAX, u32::MAX, 2047, 0, 0, 4294705152, u32::MAX],
        vec![10; 27],
        vec![
            4293918721,
            u32::MAX,
            u32::MAX,
            0x7fffffff,
            4261412864,
            u32::MAX,
            4291035135,
            4294967231,
            1049102,
            0x20000000,
            0,
            4293914624,
            0x1fffffe,
            0x80000000,
            0x8000000,
            2048,
            4294966784,
            4294966271,
            4294705151,
            u32::MAX - 1,
            0x20000,
            0x80000000,
            2047,
            0,
            0,
            4294705152,
            u32::MAX,
        ],
    );
    // - asm1[n] != 0
    test(
        vec![3338024033, 1570788701, 4067509056, 680440343],
        vec![599772085, 925834366],
        vec![10; 6],
        vec![1056633749, 686831275, 2758938475, 3727232403, 1859912609, 146677497],
    );
    // - as1[n] == 3
    test(
        vec![4030415682, 3643742328, 2586387240, 3719633661],
        vec![708497006, 797041707],
        vec![10; 6],
        vec![4203348572, 3202027474, 4170951291, 2012723103, 3609216593, 690273745],
    );
    // - slice_test_zero(&ys_0[t..]) && limbs_cmp_same_length(&ys_0[..t], ys_1) == Less
    test(
        vec![
            0,
            u32::MAX,
            u32::MAX,
            0xfffff,
            0xfffffff0,
            u32::MAX,
            63,
            0,
            0x80000000,
            u32::MAX,
            u32::MAX,
        ],
        vec![0xffff, 0, 0, 4294967264],
        vec![10; 15],
        vec![
            0,
            4294901761,
            u32::MAX,
            4293918719,
            4293918783,
            u32::MAX - 1,
            4265607103,
            1049087,
            0x7ffffff0,
            4294932480,
            63,
            0xffff,
            2147483664,
            u32::MAX,
            4294967263,
        ],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_42_fail_1() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_42_scratch_len(1, 1)];
    let mut out = vec![10; 4];
    limbs_mul_greater_to_out_toom_42(&mut out, &[6], &[1], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_42_fail_2() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_42_scratch_len(5, 6)];
    let mut out = vec![10; 11];
    let xs = series(6, 5);
    let ys = series(1, 6);
    limbs_mul_greater_to_out_toom_42(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_42_fail_3() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_42_scratch_len(3, 2)];
    let mut out = vec![10; 9];
    let xs = series(6, 3);
    limbs_mul_greater_to_out_toom_42(&mut out, &xs, &[1, 2], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_42_fail_4() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_42_scratch_len(5, 0)];
    let mut out = vec![10; 5];
    let xs = series(6, 5);
    limbs_mul_greater_to_out_toom_42(&mut out, &xs, &[], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_42_fail_5() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_42_scratch_len(4, 2)];
    let mut out = vec![10; 4];
    let xs = series(6, 4);
    limbs_mul_greater_to_out_toom_42(&mut out, &xs, &[1, 2], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mul_greater_to_out_toom_43() {
    let test = |xs: Vec<Limb>, ys: Vec<Limb>, out_before: Vec<Limb>, out_after| {
        let mut out = out_before.to_vec();
        limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys);
        assert_eq!(out, out_after);
        let mut out = out_before.to_vec();
        let mut scratch = vec![0; limbs_mul_greater_to_out_toom_43_scratch_len(xs.len(), ys.len())];
        limbs_mul_greater_to_out_toom_43(&mut out, &xs, &ys, &mut scratch);
        assert_eq!(out, out_after);
    };
    // - n_high < n in limbs_mul_toom_evaluate_deg_3poly_in_2and_neg_2
    // - !v_neg_2neg in limbs_mul_toom_evaluate_deg_3poly_in_2and_neg_2
    // - limbs_cmp_same_length(small_scratch, bsm1) != Less
    // - s <= t
    // - !v_neg_2neg in limbs_mul_toom_interpolate_6points
    // - !v_neg_1_neg in limbs_mul_toom_interpolate_6points
    // - n_high > n in limbs_mul_toom_interpolate_6points
    // - special_carry_1 <= special_carry_2
    test(
        series(2, 11),
        series(3, 9),
        vec![10; 20],
        vec![
            6, 17, 34, 58, 90, 131, 182, 244, 318, 381, 444, 468, 476, 467, 440, 394, 328, 241,
            132, 0,
        ],
    );
    // - n_high >= n in limbs_mul_toom_evaluate_deg_3poly_in_2and_neg_2
    // - v_neg_2neg in limbs_mul_toom_evaluate_deg_3poly_in_2and_neg_2
    // - t != n
    // - limbs_cmp_same_length(small_scratch, bsm1) == Less
    // - *bsm1last == 0 && limbs_cmp_same_length(bsm1init, ys_1) == Less
    // - s > t
    test(
        series(2, 12),
        series(3, 8),
        vec![10; 20],
        vec![
            6, 17, 34, 58, 90, 131, 182, 244, 296, 348, 400, 452, 462, 455, 430, 386, 322, 237,
            130, 0,
        ],
    );
    // - v_neg_2neg in limbs_mul_toom_interpolate_6points
    // - v_neg_1_neg in limbs_mul_toom_interpolate_6points
    // - n_high <= n in limbs_mul_toom_interpolate_6points
    test(
        series(2, 19),
        series(3, 11),
        vec![10; 30],
        vec![
            6, 17, 34, 58, 90, 131, 182, 244, 318, 405, 506, 594, 682, 770, 858, 946, 1034, 1122,
            1210, 1235, 1236, 1212, 1162, 1085, 980, 846, 682, 487, 260, 0,
        ],
    );
    // - special_carry_1 > special_carry_2
    test(
        vec![
            3785023459, 4249117725, 1551102690, 4239134101, 2264608302, 1455009194, 3261002629,
            2233313730, 3807192178, 2550029068, 1259253479, 2657422450,
        ],
        vec![
            2921127090, 3493254221, 1579329255, 2624469567, 1678656523, 1653055771, 493445097,
            1702866165, 1046762910,
        ],
        vec![10; 21],
        vec![
            3169501142, 3910307595, 310092603, 1408815552, 1786334527, 2452212521, 670758829,
            4142968613, 1110881016, 3529286248, 2119180760, 3066268191, 1902231557, 1262478906,
            4083142666, 784312035, 3990199726, 3180402195, 1845375516, 421486236, 647662966,
        ],
    );
    test(
        vec![
            0,
            0,
            0,
            4286578688,
            u32::MAX,
            u32::MAX,
            u32::MAX,
            u32::MAX,
            u32::MAX,
            u32::MAX,
            7,
            0,
            0,
            0,
            0,
            0xfffffff0,
            u32::MAX,
            u32::MAX,
            u32::MAX,
        ],
        vec![
            0x7fffffff,
            0xfffff000,
            0x1fffff,
            0,
            0,
            0,
            2147483520,
            0,
            0xfffffff0,
            u32::MAX,
            u32::MAX,
            4290789375,
            u32::MAX,
            u32::MAX,
            u32::MAX,
            u32::MAX,
        ],
        vec![10; 35],
        vec![
            0,
            0,
            0,
            0x800000,
            4290772992,
            7,
            0xfffff000,
            u32::MAX,
            u32::MAX,
            0x3fffffff,
            4290772984,
            134184963,
            0x1000000,
            0,
            0,
            8176,
            64504,
            4261412868,
            4294967167,
            2139095038,
            0xfffff000,
            4263643135,
            0xfffffff7,
            255,
            0,
            2147483520,
            66846728,
            0xfffffff0,
            u32::MAX,
            u32::MAX,
            4290789375,
            4294967279,
            u32::MAX,
            u32::MAX,
            u32::MAX,
        ],
    );
}

#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_limbs_mul_greater_to_out_toom_43() {
    let test = |xs: Vec<Limb>, ys: Vec<Limb>, out_before: Vec<Limb>, out_after| {
        let mut out = out_before.to_vec();
        limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys);
        assert_eq!(out, out_after);
        let mut out = out_before.to_vec();
        let mut scratch = vec![0; limbs_mul_greater_to_out_toom_43_scratch_len(xs.len(), ys.len())];
        limbs_mul_greater_to_out_toom_43(&mut out, &xs, &ys, &mut scratch);
        assert_eq!(out, out_after);
    };
    test(
        vec![
            18446744073701163071,
            u64::MAX,
            u64::MAX,
            0xfffffffff,
            0,
            0,
            0,
            0,
            18446743936270598144,
            u64::MAX,
            u64::MAX,
            u64::MAX,
            0x3ffff,
            0,
            0,
            0,
            0xffff000000000000,
            0x7fff,
            0,
            0,
            18446744073709518848,
            u64::MAX,
            u64::MAX,
            u64::MAX,
            u64::MAX,
        ],
        vec![
            18437736874454810624,
            0xfffff,
            0,
            18446744039349813248,
            u64::MAX,
            u64::MAX,
            u64::MAX,
            0x7fffffffffff,
            0,
            0xfffffffffffffff0,
            u64::MAX,
            u64::MAX,
            0x7ffffff,
            18446744056529682432,
            u64::MAX,
            u64::MAX,
        ],
        vec![10; 41],
        vec![
            17879290520660869120,
            18446735277682593791,
            u64::MAX,
            288228211488194559,
            72057594004373504,
            0,
            0,
            8866461766385536,
            18446744073709551552,
            18302628885835021327,
            u64::MAX,
            0x7ffff,
            18445617082746798336,
            144114380622004095,
            0,
            9214364837600034816,
            18446744073700114495,
            2336462208959,
            34359738336,
            0x1000000000,
            18445618173803233282,
            18446744039345618958,
            127,
            0x4000000000000,
            4611721063213039616,
            18437736874454810624,
            0x7ffff,
            13835058055282163712,
            18446744039350075391,
            4398047033343,
            18446181123756392448,
            u64::MAX,
            18446598938174685183,
            562949953454079,
            0xfffffffffffffff0,
            u64::MAX,
            18446744073709518847,
            0x7ffffff,
            18446744056529682432,
            u64::MAX,
            u64::MAX,
        ],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_43_fail_1() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_43_scratch_len(1, 1)];
    let mut out = vec![10; 4];
    limbs_mul_greater_to_out_toom_43(&mut out, &[6], &[1], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_43_fail_2() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_43_scratch_len(11, 12)];
    let mut out = vec![10; 23];
    let xs = series(3, 11);
    let ys = series(2, 12);
    limbs_mul_greater_to_out_toom_43(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_43_fail_3() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_43_scratch_len(11, 8)];
    let mut out = vec![10; 19];
    let xs = series(3, 11);
    let ys = series(2, 8);
    limbs_mul_greater_to_out_toom_43(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_43_fail_4() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_43_scratch_len(12, 0)];
    let mut out = vec![10; 12];
    let xs = series(3, 11);
    limbs_mul_greater_to_out_toom_43(&mut out, &xs, &[], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_43_fail_5() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_43_scratch_len(4, 2)];
    let mut out = vec![10; 5];
    let xs = series(3, 10);
    let ys = series(2, 10);
    limbs_mul_greater_to_out_toom_43(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mul_greater_to_out_toom_44() {
    let test = |xs: Vec<Limb>, ys: Vec<Limb>, out_before: Vec<Limb>, out_after| {
        let mut out = out_before.to_vec();
        limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys);
        assert_eq!(out, out_after);
        let mut out = out_before.to_vec();
        let mut scratch = vec![0; limbs_mul_greater_to_out_toom_44_scratch_len(xs.len(), ys.len())];
        limbs_mul_greater_to_out_toom_44(&mut out, &xs, &ys, &mut scratch);
        assert_eq!(out, out_after);
    };
    // - limbs_mul_toom_evaluate_deg_3poly_in_2and_neg_2(bpx, bmx, ys, n, t, &mut scratch[..n + 1])
    // - limbs_mul_toom_evaluate_deg_3poly_in_1and_neg_1(bpx, bmx, ys, n, t, &mut scratch[..n + 1])
    // - s <= t
    // - !w1_neg
    // - !w3neg
    // - w6n <= n + 1
    // - limbs_mul_greater_to_out_basecase in limbs_mul_same_length_to_out_toom_44recursive
    test(
        series(2, 4),
        series(3, 4),
        vec![10; 8],
        vec![6, 17, 34, 58, 58, 49, 30, 0],
    );
    // - w3neg
    test(
        vec![0, 0, 0, 1],
        vec![0, 0, 1, 1],
        vec![10; 8],
        vec![0, 0, 0, 0, 0, 1, 1, 0],
    );
    // - w6n > n + 1
    test(
        vec![
            1528859315, 4288784328, 3677151116, 445199233, 3304488688, 3566979465, 3541025426,
            2491779846, 3112990742, 2583249486, 3403111749, 1930721237,
        ],
        vec![
            2700212626, 3890522506, 1407330442, 2072012244, 292784856, 2848511017, 2011019434,
            3729188240, 1314875514, 1752114201, 3480385261, 1532349465,
        ],
        vec![10; 24],
        vec![
            301610262, 3665600695, 2790869988, 562719619, 254881625, 3646308155, 2857045174,
            4219173388, 3417896791, 458617279, 3882403287, 617740409, 3296542840, 435168928,
            3570119313, 863483077, 2646855475, 2878510649, 4228994627, 2357119023, 2589237669,
            2274199643, 3000367783, 688838692,
        ],
    );
    // - s > t
    test(
        vec![
            1588217107, 79108222, 2883552792, 2390312777, 1587172303, 2070384343, 2265280181,
            4013380367,
        ],
        vec![3177381025, 2776698917, 954518943, 3785176644, 3521195169, 550485155, 1499535299],
        vec![10; 15],
        vec![
            2639930611, 1074195093, 3974952249, 2825437951, 3084912647, 2589723741, 1008656003,
            3022162475, 2305314017, 1619919364, 894905935, 3957960884, 814161571, 756465381,
            1401222667,
        ],
    );
    // - w1_neg
    test(
        vec![
            1047248630, 339306853, 1100911694, 3907715577, 4281628442, 1447091409, 3425204321,
            3871347591, 339462242, 1765234031, 3774533011, 980706746,
        ],
        vec![
            1454868694, 1975460471, 2212752551, 1982786615, 983847073, 3073742136, 438698610,
            1215648998, 2824467771, 3299124311, 2818671068,
        ],
        vec![10; 23],
        vec![
            2438877604, 4249888081, 2301349363, 1817920534, 2538709343, 1739256708, 179543633,
            2275519806, 1688820820, 759475921, 3927834077, 2138533648, 958932069, 2429920287,
            3858014276, 2853106604, 1837491388, 1616377262, 231659922, 680814190, 417532392,
            428918230, 643611358,
        ],
    );
    test(
        vec![986333060, 254638637, 1577120658, 1458096412, 474582958, 4115735719, 4031007047],
        vec![2096725444, 3871299248, 1414038108, 2617834141, 1553210626, 2669030715, 3093885541],
        vec![10; 14],
        vec![
            2067797264, 3922708625, 2600678884, 825822853, 2499590824, 1035492325, 1957325707,
            1890833276, 3433274404, 1510974136, 2269171082, 854613327, 1796482159, 2903741417,
        ],
    );
}

#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_limbs_mul_greater_to_out_toom_44() {
    let test = |xs: Vec<Limb>, ys: Vec<Limb>, out_before: Vec<Limb>, out_after| {
        let mut out = out_before.to_vec();
        limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys);
        assert_eq!(out, out_after);
        let mut out = out_before.to_vec();
        let mut scratch = vec![0; limbs_mul_greater_to_out_toom_44_scratch_len(xs.len(), ys.len())];
        limbs_mul_greater_to_out_toom_44(&mut out, &xs, &ys, &mut scratch);
        assert_eq!(out, out_after);
    };
    test(
        series(2, 4),
        series(3, 4),
        vec![10; 8],
        vec![6, 17, 34, 58, 58, 49, 30, 0],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_44_fail_1() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_44_scratch_len(1, 1)];
    let mut out = vec![10; 10];
    limbs_mul_greater_to_out_toom_44(&mut out, &[6], &[1], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_44_fail_2() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_44_scratch_len(4, 4)];
    let mut out = vec![10; 9];
    let xs = series(3, 4);
    let ys = series(2, 5);
    limbs_mul_greater_to_out_toom_44(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_44_fail_3() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_44_scratch_len(3, 3)];
    let mut out = vec![10; 6];
    let xs = series(3, 3);
    let ys = series(2, 3);
    limbs_mul_greater_to_out_toom_44(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_44_fail_4() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_44_scratch_len(11, 11)];
    let mut out = vec![10; 11];
    let xs = series(3, 11);
    limbs_mul_greater_to_out_toom_44(&mut out, &xs, &[], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_44_fail_5() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_44_scratch_len(4, 4)];
    let mut out = vec![10; 7];
    let xs = series(3, 4);
    let ys = series(2, 4);
    limbs_mul_greater_to_out_toom_44(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mul_greater_to_out_toom_52() {
    let test = |xs: &[Limb], ys: &[Limb], out_before: &[Limb], out_after: &[Limb]| {
        let mut out = out_before.to_vec();
        limbs_mul_greater_to_out_basecase(&mut out, xs, ys);
        assert_eq!(out, out_after);
        let mut out = out_before.to_vec();
        let mut scratch = vec![0; limbs_mul_greater_to_out_toom_52_scratch_len(xs.len(), ys.len())];
        limbs_mul_greater_to_out_toom_52(&mut out, xs, ys, &mut scratch);
        assert_eq!(out, out_after);
    };
    // - degree.even() in limbs_mul_toom_evaluate_poly_in_2and_neg_2
    // - !v_neg_2neg in limbs_mul_toom_evaluate_poly_in_2and_neg_2
    // - t != n
    // - !(slice_test_zero(&ys_0[t..]) && limbs_cmp_same_length(&ys_0[..t], ys_1) == Less)
    // - !v_neg_1_neg
    // - !(slice_test_zero(&bsm1[t..]) && limbs_cmp_same_length(&bsm1[..t], ys_1) == Less)
    // - degree.even() in limbs_mul_toom_evaluate_poly_in_1and_neg_1
    // - !v_neg_1_neg in limbs_mul_toom_evaluate_poly_in_1and_neg_1
    test(
        &series(2, 15),
        &series(3, 5),
        &[10; 20],
        &[
            6, 17, 34, 58, 90, 115, 140, 165, 190, 215, 240, 265, 290, 315, 340, 314, 268, 201,
            112, 0,
        ],
    );
    test(
        &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        &[0, 0, 0, 0, 1, 0],
        &[10; 20],
        &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
    );
    // - n_high != n in limbs_mul_toom_evaluate_poly_in_2and_neg_2
    // - t == n
    // - limbs_cmp_same_length(ys_0, ys_1) == Less
    // - v_neg_1_neg
    test(
        &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        &[0, 0, 0, 0, 0, 1],
        &[10; 20],
        &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    );
    // - slice_test_zero(&ys_0[t..]) && limbs_cmp_same_length(&ys_0[..t], ys_1) == Less
    test(
        &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        &[0, 0, 0, 0, 1],
        &[10; 20],
        &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    );
    // - limbs_cmp_same_length(ys_0, ys_1) != Less
    // - limbs_cmp_same_length(bsm1, ys_1) == Less
    test(
        &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        &[0, 0, 1, 0, 0, 1],
        &[10; 20],
        &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0],
    );
    // - v_neg_2neg in limbs_mul_toom_evaluate_poly_in_2and_neg_2
    // - v_neg_1_neg in limbs_mul_toom_evaluate_poly_in_1and_neg_1
    // - limbs_mul_toom_evaluate_poly_in_1and_neg_1(as1, asm1, 4, xs, n, s, &mut v_neg_1[..m])
    test(
        &[
            281500646, 1572406350, 108746052, 4056047843, 89307364, 1006007374, 2902260577,
            1250995384, 1556873818, 3846421711, 280743259, 1728158805, 467926284, 2330565417,
        ],
        &[2509320863, 2201587434, 926371577, 1243694325, 1112023631, 2791032478],
        &[10; 20],
        &[
            1191903194, 1277561191, 2672986331, 45667421, 2742410814, 2602170945, 2815699572,
            2317624023, 952805243, 577394769, 1002744907, 4175910221, 2433548489, 2550394831,
            3650814344, 1121996596, 3441179979, 3561879910, 1574546788, 1514489709,
        ],
    );
    // - limbs_cmp_same_length(bsm1, ys_1) != Less
    test(
        &[
            2331447040, 1003213663, 1873981685, 3371337621, 3796896013, 4144448610, 2569252563,
            2859304641, 1027973602, 3158196152, 4058699545, 2002924383, 3295505824, 695758308,
        ],
        &[725028139, 2984864771, 2939417227, 3047223286, 3526157986, 1078000342],
        &[10; 20],
        &[
            474121472, 1561322164, 715684992, 3182777436, 384530074, 3827205870, 2267366778,
            1586160630, 3779201468, 900553139, 2867049131, 2027414411, 2054056558, 2671776484,
            3374007062, 3091178442, 1888125000, 2974781424, 307612679, 174629431,
        ],
    );
    // - slice_test_zero(&bsm1[t..]) && limbs_cmp_same_length(&bsm1[..t], ys_1) == Less
    test(
        &rle_decode(&[
            (32767, 1),
            (0, 5),
            (4294836224, 1),
            (u32::MAX, 5),
            (4278206463, 1),
            (u32::MAX, 2),
            (31, 1),
            (0, 2),
            (4294443008, 1),
            (u32::MAX, 7),
        ]),
        &[0, 0, u32::MAX, u32::MAX, u32::MAX, 0, 0, 4294967232, u32::MAX, 4227858559, u32::MAX],
        &[10; 37],
        &[
            0,
            0,
            4294934529,
            u32::MAX,
            u32::MAX,
            0x7ffe,
            0,
            4292870208,
            0x1ffff,
            71303040,
            4294966784,
            4294868990,
            u32::MAX,
            0x7fffff,
            16760832,
            0xff000000,
            2047,
            4278075360,
            u32::MAX,
            1072693247,
            524320,
            2149580800,
            259839,
            4277682176,
            2147487743,
            0x1ffffff,
            32,
            4227858432,
            8190,
            4294443008,
            u32::MAX,
            0,
            0,
            4294967232,
            u32::MAX,
            4227858559,
            u32::MAX,
        ],
    );
}

#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_limbs_mul_greater_to_out_toom_52() {
    let test = |xs: Vec<Limb>, ys: Vec<Limb>, out_before: Vec<Limb>, out_after| {
        let mut out = out_before.to_vec();
        limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys);
        assert_eq!(out, out_after);
        let mut out = out_before.to_vec();
        let mut scratch = vec![0; limbs_mul_greater_to_out_toom_52_scratch_len(xs.len(), ys.len())];
        limbs_mul_greater_to_out_toom_52(&mut out, &xs, &ys, &mut scratch);
        assert_eq!(out, out_after);
    };
    test(
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![0, 0, 0, 0, 1, 0],
        vec![10; 20],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_52_fail_1() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_52_scratch_len(1, 1)];
    let mut out = vec![10; 4];
    limbs_mul_greater_to_out_toom_52(&mut out, &[6], &[1], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_52_fail_2() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_52_scratch_len(15, 16)];
    let mut out = vec![10; 9];
    let xs = series(3, 15);
    let ys = series(3, 16);
    limbs_mul_greater_to_out_toom_52(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_52_fail_3() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_52_scratch_len(14, 5)];
    let mut out = vec![10; 6];
    let xs = series(3, 14);
    let ys = series(3, 5);
    limbs_mul_greater_to_out_toom_52(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_52_fail_4() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_52_scratch_len(15, 4)];
    let mut out = vec![10; 7];
    let xs = series(3, 15);
    let ys = series(3, 4);
    limbs_mul_greater_to_out_toom_52(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_52_fail_5() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_52_scratch_len(11, 0)];
    let mut out = vec![10; 12];
    let xs = series(3, 11);
    limbs_mul_greater_to_out_toom_52(&mut out, &xs, &[], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mul_greater_to_out_toom_53() {
    let test = |xs: Vec<Limb>, ys: Vec<Limb>, out_before: Vec<Limb>, out_after| {
        let mut out = out_before.to_vec();
        limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys);
        assert_eq!(out, out_after);
        let mut out = out_before.to_vec();
        let mut scratch = vec![0; limbs_mul_greater_to_out_toom_53_scratch_len(xs.len(), ys.len())];
        limbs_mul_greater_to_out_toom_53(&mut out, &xs, &ys, &mut scratch);
        assert_eq!(out, out_after);
    };
    // - !(*bs1last == 0 && limbs_cmp_same_length(bs1init, ys_1) == Less)
    // - limbs_cmp_same_length(bs2, &out[..n + 1]) != Less
    // - *asm1last != 1 && *asm1last != 2
    // - *as1last == 0
    test(
        series(2, 5),
        series(3, 3),
        vec![10; 8],
        vec![6, 17, 34, 46, 58, 49, 30, 0],
    );
    // - *bs1last == 0 && limbs_cmp_same_length(bs1init, ys_1) == Less
    // - *as1last == 2
    // - *bs1last == 1
    test(
        vec![611094747, 2426195984, 3948451542, 3575143460, 2163084716],
        vec![1043494367, 2432724646, 1148376235],
        vec![10; 8],
        vec![
            2911651269, 2135822080, 566305911, 1285474929, 3971527858, 1120629777, 2330897846,
            578359487,
        ],
    );
    // - *as1last == 1
    test(
        vec![83336617, 52963853, 1461131367, 615175494, 2376138249],
        vec![1085015601, 823246134, 3222784883],
        vec![10; 8],
        vec![
            4003668825, 3129188105, 1975732797, 2019047981, 943873016, 1483316813, 305883771,
            1782966412,
        ],
    );
    // - limbs_cmp_same_length(bs2, &out[..n + 1]) == Less
    // - *as1last > 2
    test(
        vec![
            3853679659, 4236745288, 2469732913, 4265854402, 4207372271, 1754370134, 137881047,
            1325109821, 2212043812, 3074170203,
        ],
        vec![1666773246, 4177391250, 4175984066, 2859904653, 3320165100, 314964734],
        vec![10; 16],
        vec![
            2336719530, 919351696, 4142757378, 49781824, 1315087108, 2534950116, 2674417418,
            1178559126, 171926136, 3132896187, 2074730624, 3561766617, 1155879861, 3985534229,
            380101898, 225439482,
        ],
    );
    // - *asm1last == 1
    test(
        vec![4171807709, 1363035595, 2692148345, 3728232161, 2672522097],
        vec![178202067, 736149219, 623937260],
        vec![10; 8],
        vec![
            2793195559, 2168235304, 1582195836, 18437203, 671570200, 635034059, 2378259056,
            388241865,
        ],
    );
    // - *bs1last == 2
    test(
        vec![361692441, 3665267779, 1770324312, 1221560416, 2810295690],
        vec![1887715703, 4035171395, 2815003797],
        vec![10; 8],
        vec![
            3298754463, 2516900264, 30373680, 909364693, 729609199, 3973437903, 3392713387,
            1841921601,
        ],
    );
    // - *bsm1last != 0
    test(
        vec![1542637461, 595638956, 1883922642, 2681579369, 2641006916],
        vec![3723002977, 116606811, 2193352864],
        vec![10; 8],
        vec![
            2246996853, 3232877055, 2347711939, 2476049074, 4132376421, 3855440382, 4040315714,
            1348708775,
        ],
    );
    // - *asm1last == 2
    test(
        vec![4043423637, 312331403, 3088235367, 41162462, 2934893364],
        vec![2702987034, 4184574368, 2455116868],
        vec![10; 8],
        vec![
            2912448546, 2297161059, 137328692, 115875329, 1975003140, 2441893159, 4034859213,
            1677662647,
        ],
    );
    test(
        vec![0x3ffff8, 3221225472, u32::MAX, 1, 4294934528],
        vec![0, 4294959104, u32::MAX],
        vec![10; 8],
        vec![0, 0x10000, 0xfffffff8, 4196343, 3221209088, 0xfffffff, 4294959106, 4294934527],
    );
}

#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_limbs_mul_greater_to_out_toom_53() {
    let test = |xs: Vec<Limb>, ys: Vec<Limb>, out_before: Vec<Limb>, out_after| {
        let mut out = out_before.to_vec();
        limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys);
        assert_eq!(out, out_after);
        let mut out = out_before.to_vec();
        let mut scratch = vec![0; limbs_mul_greater_to_out_toom_53_scratch_len(xs.len(), ys.len())];
        limbs_mul_greater_to_out_toom_53(&mut out, &xs, &ys, &mut scratch);
        assert_eq!(out, out_after);
    };
    test(
        vec![u64::MAX, u64::MAX, 3, u64::MAX - 1, u64::MAX],
        vec![u64::MAX, u64::MAX, u64::MAX],
        vec![10; 8],
        vec![1, 0, 0xfffffffffffffffc, 0, 0, 3, u64::MAX - 1, u64::MAX],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_53_fail_1() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_53_scratch_len(1, 1)];
    let mut out = vec![10; 4];
    limbs_mul_greater_to_out_toom_53(&mut out, &[6], &[1], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_53_fail_2() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_53_scratch_len(5, 6)];
    let mut out = vec![10; 11];
    let xs = series(3, 5);
    let ys = series(3, 6);
    limbs_mul_greater_to_out_toom_53(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_53_fail_3() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_53_scratch_len(5, 4)];
    let mut out = vec![10; 9];
    let xs = series(3, 5);
    let ys = series(3, 4);
    limbs_mul_greater_to_out_toom_53(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_53_fail_4() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_53_scratch_len(4, 3)];
    let mut out = vec![10; 6];
    let xs = series(3, 4);
    let ys = series(3, 3);
    limbs_mul_greater_to_out_toom_53(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_53_fail_5() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_53_scratch_len(5, 2)];
    let mut out = vec![10; 7];
    let xs = series(3, 5);
    limbs_mul_greater_to_out_toom_53(&mut out, &xs, &[3, 4], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_53_fail_6() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_53_scratch_len(5, 0)];
    let mut out = vec![10; 12];
    let xs = series(3, 5);
    limbs_mul_greater_to_out_toom_53(&mut out, &xs, &[], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mul_greater_to_out_toom_54() {
    let test = |xs: Vec<Limb>, ys: Vec<Limb>, out_before: Vec<Limb>, out_after| {
        let mut out = out_before.to_vec();
        limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys);
        assert_eq!(out, out_after);
        let mut out = out_before.to_vec();
        let mut scratch = vec![0; limbs_mul_greater_to_out_toom_54_scratch_len(xs.len(), ys.len())];
        limbs_mul_greater_to_out_toom_54(&mut out, &xs, &ys, &mut scratch);
        assert_eq!(out, out_after);
    };
    // - degree.even() in limbs_mul_toom_evaluate_poly_in_2pow_and_neg_2pow
    // - !v_neg_2pow_neg in limbs_mul_toom_evaluate_poly_in_2pow_and_neg_2pow
    // - degree.odd() in limbs_mul_toom_evaluate_poly_in_2pow_and_neg_2pow
    // - !y_sign in limbs_toom_couple_handling
    // - y_shift != 0 in limbs_toom_couple_handling
    // - x_shift != 0 in limbs_toom_couple_handling
    // - s > t
    // - carry_1 && !carry_2, first time, in limbs_mul_toom_interpolate_8points
    // - carry_1 && !carry_2, second time, in limbs_mul_toom_interpolate_8points
    // - s_plus_t != n in limbs_mul_toom_interpolate_8points
    test(
        series(2, 15),
        series(3, 11),
        vec![10; 26],
        vec![
            6, 17, 34, 58, 90, 131, 182, 244, 318, 405, 506, 594, 682, 770, 858, 895, 912, 908,
            882, 833, 760, 662, 538, 387, 208, 0,
        ],
    );
    // - v_neg_2pow_neg in limbs_mul_toom_evaluate_poly_in_2pow_and_neg_2pow
    // - y_sign in limbs_toom_couple_handling
    // - s <= t
    test(
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
        vec![10; 26],
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
    );
    // - !carry_1 && carry_2, first time, in limbs_mul_toom_interpolate_8points
    test(
        vec![
            281500646, 1572406350, 108746052, 4056047843, 89307364, 1006007374, 2902260577,
            1250995384, 1556873818, 3846421711, 280743259, 1728158805, 467926284, 2330565417,
        ],
        vec![
            1365578038, 3224231142, 4103857906, 475734936, 3828952167, 3071966456, 1450111251,
            1166414077, 2218130537, 3324650407, 1559641024, 2423373264,
        ],
        vec![10; 26],
        vec![
            3471157380, 2179990259, 735116018, 3928626279, 2606792426, 4065628313, 3326106964,
            1358767242, 58836620, 2388814047, 1881937395, 448453590, 699295041, 2539838591,
            1014080982, 2627397171, 1231543630, 2956184941, 1108982880, 2083442227, 1445361702,
            3773463966, 3902311612, 4169089467, 614631841, 1314987876,
        ],
    );
    // - s_plus_t == n in limbs_mul_toom_interpolate_8points
    test(
        vec![
            1372428912, 2999825770, 3824933735, 1252466299, 1644332514, 577056155, 267504800,
            2188417248, 1146838357, 1601878440, 2555350187, 2326995902, 341200833, 3311243465,
            3983323172, 1591023018, 498264278, 879686658, 2445286712, 3168806215, 3363960673,
            1002293448,
        ],
        vec![
            4155394173, 3251572031, 3012777338, 1405107169, 4263655764, 202386116, 2762119705,
            1046271690, 3730474184, 1761497041, 3530189728, 452831577, 2351117985, 3074633806,
            2337874996, 2372535352, 1907593160, 2034262144,
        ],
        vec![10; 40],
        vec![
            3438536880, 4020840252, 3753658662, 2750457729, 3984463794, 1677702279, 3627178035,
            1938289829, 2347934241, 1059164524, 3077109858, 1455283397, 4245424824, 2265496611,
            2507273589, 4106853892, 187386657, 3541881161, 3520589236, 977961476, 205850208,
            3040196950, 1303835716, 3039701923, 525989195, 1042461957, 4189151284, 3358396344,
            275215531, 2907721257, 3086020483, 2914223316, 652103889, 2430562590, 4256409533,
            774831877, 3808631269, 3720895601, 1034939105, 474724830,
        ],
    );
    // - !carry_1 && carry_2, second time, in limbs_mul_toom_interpolate_8points
    test(
        vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xfffffff0, u32::MAX, u32::MAX, u32::MAX],
        vec![
            2047,
            0,
            0,
            4294966784,
            u32::MAX,
            127,
            0,
            4286578688,
            u32::MAX,
            0x3ffff,
            4227858432,
            u32::MAX,
        ],
        vec![10; 26],
        vec![
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            4294934544,
            u32::MAX,
            u32::MAX,
            8191,
            2047,
            4294965248,
            u32::MAX,
            134217215,
            0,
            4290773120,
            0x3fffffff,
            4286578688,
            4294967279,
            0x3ffff,
            4227858432,
            u32::MAX,
        ],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_54_fail_1() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_54_scratch_len(1, 1)];
    let mut out = vec![10; 4];
    limbs_mul_greater_to_out_toom_54(&mut out, &[6], &[1], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_54_fail_2() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_54_scratch_len(15, 16)];
    let mut out = vec![10; 31];
    let xs = series(3, 14);
    let ys = series(3, 17);
    limbs_mul_greater_to_out_toom_54(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_54_fail_3() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_54_scratch_len(15, 10)];
    let mut out = vec![10; 25];
    let xs = series(3, 14);
    let ys = series(3, 10);
    limbs_mul_greater_to_out_toom_54(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_54_fail_4() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_54_scratch_len(14, 11)];
    let mut out = vec![10; 25];
    let xs = series(3, 14);
    let ys = series(3, 11);
    limbs_mul_greater_to_out_toom_54(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_54_fail_5() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_54_scratch_len(15, 11)];
    let mut out = vec![10; 25];
    let xs = series(3, 15);
    let ys = series(3, 11);
    limbs_mul_greater_to_out_toom_54(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_54_fail_6() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_54_scratch_len(15, 0)];
    let mut out = vec![10; 15];
    let xs = series(3, 15);
    limbs_mul_greater_to_out_toom_54(&mut out, &xs, &[], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mul_greater_to_out_toom_62() {
    let test = |xs: &[Limb], ys: &[Limb], out_before: &[Limb], out_after: &[Limb]| {
        let mut out = out_before.to_vec();
        limbs_mul_greater_to_out_basecase(&mut out, xs, ys);
        assert_eq!(out, out_after);
        let mut out = out_before.to_vec();
        let mut scratch = vec![0; limbs_mul_greater_to_out_toom_62_scratch_len(xs.len(), ys.len())];
        limbs_mul_greater_to_out_toom_62(&mut out, xs, ys, &mut scratch);
        assert_eq!(out, out_after);
    };
    // - degree.odd() in limbs_mul_toom_evaluate_poly_in_1and_neg_1
    // - degree > 4 in limbs_mul_toom_evaluate_poly_in_1and_neg_1
    // - degree_u >= 5 in limbs_mul_toom_evaluate_poly_in_2and_neg_2
    // - degree.odd() in limbs_mul_toom_evaluate_poly_in_2and_neg_2
    // - t == n
    // - limbs_cmp_same_length(ys_0, ys_1) == Less
    // - v_neg_1_neg_b
    // - *as1last == 0
    test(
        &series(2, 6),
        &[3, 4],
        &[10; 8],
        &[6, 17, 24, 31, 38, 45, 28, 0],
    );
    // - limbs_cmp_same_length(ys_0, ys_1) != Less
    // - !v_neg_1_neg_b
    // - t >= n
    // - limbs_cmp_same_length(&bsm1, ys_1) == Less
    test(
        &[0, 0, 0, 0, 0, 1],
        &[1, 1],
        &[10; 8],
        &[0, 0, 0, 0, 0, 1, 1, 0],
    );
    // - limbs_cmp_same_length(&bsm1, ys_1) != Less
    test(
        &[0, 0, 0, 0, 0, 1],
        &[2, 1],
        &[10; 8],
        &[0, 0, 0, 0, 0, 2, 1, 0],
    );
    // - t != n
    // - !(slice_test_zero(&ys_0[t..]) && limbs_cmp_same_length(&ys_0[..t], ys_1) == Less)
    // - t < n
    // - !(slice_test_zero(&bsm1[t..]) && limbs_cmp_same_length(&bsm1[..t], ys_1) == Less)
    // - *as1last == 2
    test(
        &[
            2291585918, 1380546066, 1861205162, 1395600128, 1509813785, 1715266614, 3251195596,
            3140058077, 1998653517, 3140019184, 2534426976,
        ],
        &[2477133296, 625749873, 3687467688],
        &[10; 14],
        &[
            869772320, 253774892, 3270059412, 1629301906, 333315526, 1485838973, 1182872659,
            3973435471, 3570040059, 138616924, 3845622124, 4243476600, 2488800838, 2175946157,
        ],
    );
    // - *as1last > 2
    test(
        &[706760835, 4153647095, 3843998199, 2077172825, 1158686949, 3157624247],
        &[708497006, 797041707],
        &[10; 8],
        &[
            3596223050, 1899342498, 3768933007, 59388593, 2997914214, 150267535, 1848145862,
            585978436,
        ],
    );
    // - *as1last == 1
    test(
        &[
            1817453168, 96871997, 3927306877, 3090061646, 3474317652, 437148773, 439538568,
            324686794, 772632617, 1424328970, 580538580,
        ],
        &[4158498322, 3126677346, 655989538],
        &[10; 14],
        &[
            4142861280, 2093741387, 1223409636, 3430701278, 154561385, 1065478559, 1434432315,
            1709306376, 2647162930, 2288715437, 510829208, 3519993529, 1581992297, 88668250,
        ],
    );
    // - *bs1last != 0
    test(
        &[478149678, 4026802122, 1384639138, 368837837, 183900171, 785221208],
        &[1458158767, 4167624669],
        &[10; 8],
        &[
            1921854322, 141249793, 673006993, 2183916852, 295623924, 3471440317, 3387755993,
            761939975,
        ],
    );
    // - *asm1last == 1
    test(
        &[760464004, 3698115579, 1282981837, 2124133062, 1943175022, 3815903204],
        &[2302225798, 423133196],
        &[10; 8],
        &[
            1718420760, 4288660832, 1043184986, 2518603664, 1668853787, 1047988481, 4101944437,
            375936580,
        ],
    );
    // - *asm1last == 2
    test(
        &[
            486320673, 3488920730, 3556919186, 380261964, 1609664786, 3382076763, 3478178414,
            1464325754, 2543330707, 3900552438, 1432199477,
        ],
        &[1190326122, 1081384689, 2610845505, 3919894794],
        &[10; 15],
        &[
            3164946602, 4284198222, 380177155, 837655879, 3034889727, 3503063664, 3315274214,
            3998279880, 2501466635, 3524441, 312561544, 2480833439, 3092764257, 1045878247,
            1307127829,
        ],
    );
    // - slice_test_zero(&ys_0[t..]) && limbs_cmp_same_length(&ys_0[..t], ys_1) == Less
    test(
        &rle_decode(&[(u32::MAX, 17), (31, 1), (0, 5), (4294967232, 1), (u32::MAX, 12)]),
        &[u32::MAX, 63, 0, 0, 0, 0, 4227858432, u32::MAX, u32::MAX, u32::MAX, u32::MAX],
        &[10; 47],
        &[
            1,
            4294967232,
            u32::MAX,
            u32::MAX,
            u32::MAX,
            u32::MAX,
            0x3ffffff,
            0,
            0,
            0,
            0,
            u32::MAX,
            u32::MAX,
            u32::MAX,
            u32::MAX,
            u32::MAX,
            u32::MAX,
            4294967263,
            2047,
            0,
            0,
            0,
            0,
            2147483712,
            4294963199,
            u32::MAX,
            u32::MAX,
            u32::MAX,
            31,
            0,
            1,
            0,
            0,
            0,
            4294967232,
            u32::MAX,
            u32::MAX - 1,
            63,
            0,
            0,
            0,
            0,
            4227858432,
            u32::MAX,
            u32::MAX,
            u32::MAX,
            u32::MAX,
        ],
    );
    // - slice_test_zero(&bsm1[t..]) && limbs_cmp_same_length(&bsm1[..t], ys_1) == Less
    test(
        &rle_decode(&[(1073741823, 1), (0, 7), (4290772992, 1), (u32::MAX, 8)]),
        &[0xfff00000, u32::MAX, 0, 0xffffff8, 4294443008],
        &[10; 22],
        &[
            0x100000,
            4294705152,
            0x3ffffffe,
            4026531848,
            67633149,
            1073610751,
            0,
            0,
            0,
            1024,
            4290772992,
            0x1ffffff,
            4294705152,
            4290773503,
            u32::MAX,
            u32::MAX,
            u32::MAX,
            4293918719,
            u32::MAX,
            0,
            0xffffff8,
            4294443008,
        ],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_62_fail_1() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_62_scratch_len(1, 1)];
    let mut out = vec![10; 4];
    limbs_mul_greater_to_out_toom_62(&mut out, &[6], &[1], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_62_fail_2() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_62_scratch_len(6, 7)];
    let mut out = vec![10; 13];
    let xs = series(3, 6);
    let ys = series(3, 7);
    limbs_mul_greater_to_out_toom_62(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_62_fail_3() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_62_scratch_len(6, 1)];
    let mut out = vec![10; 7];
    let xs = series(3, 6);
    limbs_mul_greater_to_out_toom_62(&mut out, &xs, &[3], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_62_fail_4() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_62_scratch_len(5, 2)];
    let mut out = vec![10; 7];
    let xs = series(3, 5);
    limbs_mul_greater_to_out_toom_62(&mut out, &xs, &[3, 4], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_62_fail_5() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_62_scratch_len(6, 2)];
    let mut out = vec![10; 7];
    let xs = series(3, 6);
    limbs_mul_greater_to_out_toom_62(&mut out, &xs, &[3, 4], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_62_fail_6() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_62_scratch_len(6, 0)];
    let mut out = vec![10; 6];
    let xs = series(3, 6);
    limbs_mul_greater_to_out_toom_62(&mut out, &xs, &[], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mul_greater_to_out_toom_63() {
    let test = |xs: Vec<Limb>, ys: Vec<Limb>, out_before: Vec<Limb>, out_after| {
        let mut out = out_before.to_vec();
        limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys);
        assert_eq!(out, out_after);
        let mut out = out_before.to_vec();
        let mut scratch = vec![0; limbs_mul_greater_to_out_toom_63_scratch_len(xs.len(), ys.len())];
        limbs_mul_greater_to_out_toom_63(&mut out, &xs, &ys, &mut scratch);
        assert_eq!(out, out_after);
    };
    // - n == t
    // - !(!carry && limbs_cmp_same_length(scratch2lo, ys_1) == Less)
    // - s <= t
    test(
        series(2, 17),
        series(3, 9),
        vec![10; 26],
        vec![
            6, 17, 34, 58, 90, 131, 182, 244, 318, 381, 444, 507, 570, 633, 696, 759, 822, 828,
            812, 773, 710, 622, 508, 367, 198, 0,
        ],
    );
    // - n != t
    test(
        vec![
            3047748962, 2284186344, 3132866461, 2331447040, 1003213663, 1873981685, 3371337621,
            3796896013, 4144448610, 2569252563, 2859304641, 1027973602, 3158196152, 4058699545,
            2002924383, 3295505824, 695758308, 544681384, 3452307839, 1190734708, 4232023153,
            451772934, 673919865, 2022672425, 3493426012, 1142609332, 477542383, 1304798841,
            461115870, 3268103575, 2243523508,
        ],
        vec![
            3987208830, 1336657961, 2605546090, 1112778759, 2243645577, 3695113963, 637209276,
            527642657, 1586863943, 2178788843, 4128924923, 574016400, 118333022, 3019059425,
            3734056582, 3974475640, 958936732,
        ],
        vec![10; 48],
        vec![
            901282364, 4131825926, 550521101, 4239081984, 354957348, 2987335611, 2947836402,
            1594339509, 1900787939, 3942224706, 1915750189, 2686147736, 455238733, 595779993,
            992449470, 225135268, 4216025815, 112446550, 2736130746, 1015352940, 1166343395,
            3559470539, 2787138552, 3128535813, 2203140859, 3479459112, 599923700, 684443693,
            1557326194, 1699057519, 2198150417, 2196463130, 1973109458, 3642841764, 426750624,
            1438683694, 42406461, 1444651840, 2152704621, 722727455, 3882030279, 205951250,
            838845869, 2997862064, 779154540, 1753953589, 1791445120, 500911172,
        ],
    );
    test(
        vec![
            2547108010, 2828666778, 3252702258, 3885923576, 2331974758, 730724707, 1528859315,
            4288784328, 3677151116, 445199233, 3304488688, 3566979465, 3541025426, 2491779846,
            3112990742, 2583249486, 3403111749, 1930721237, 3428792463, 2896462048, 2985885576,
            1819460734, 21206096, 3560441846, 987100555, 2844904275, 84854892, 1268249628,
            3963306788, 3338670067, 2504599089, 65588657, 321493327, 4249673617, 4150876068,
            721566898,
        ],
        vec![
            2339094549, 568841948, 757218994, 54206328, 2888117240, 1758638903, 3215886938,
            2041086168, 259363425, 3740850804, 3272104239, 3101597497, 4170226346, 1487680512,
            2997309052, 1761169487, 680164259, 104354801, 3642294827, 2001649447,
        ],
        vec![10; 56],
        vec![
            4156749298, 1238334534, 3541686335, 400023669, 3354392679, 146448234, 338562445,
            2541647274, 3476105410, 3869729511, 2592129633, 1524174755, 2864342013, 3189404137,
            2408966423, 1748955694, 848863232, 2061232865, 2863992687, 1780371599, 1814973544,
            4129152748, 1067034680, 3960771432, 1978132071, 249147649, 4113633238, 3331366833,
            103867284, 4274561406, 24372440, 1874890180, 2262704206, 4185039557, 1493676561,
            3605651563, 184712156, 1714079946, 3695806969, 3114374817, 2698021971, 2617815992,
            3374318018, 2710182754, 2217042831, 3166354273, 3526471084, 2282901181, 17853137,
            2805842653, 2980411632, 2879849003, 22987084, 2408312078, 212023482, 336282883,
        ],
    );
    // - !carry && limbs_cmp_same_length(scratch2lo, ys_1) == Less
    // - s > t
    test(
        vec![
            275320572, 2678313698, 1997150503, 1718206458, 3389415001, 1347098060, 423205500,
            1228674579, 1683636524, 1761485682, 3886555164, 1343770739, 3728441996, 3386212640,
            4218849286, 3154177905, 383775865, 685210915, 2915358388, 356527607, 1399377005,
            2203631586, 3950305635, 4107289625,
        ],
        vec![
            343872945, 2028904125, 1525417887, 867188532, 3911999830, 2139706847, 3256484706,
            961423019, 1530068826, 3577946967,
        ],
        vec![10; 34],
        vec![
            367134780, 454511356, 740068730, 2466817027, 444007987, 2116910983, 3588258390,
            4148666142, 241899205, 3037479671, 967522541, 1695514557, 3417684811, 1755587152,
            57889847, 1893598444, 894827452, 1259092281, 343759711, 417669929, 4250137916,
            2931151486, 4137704826, 1616987343, 118402896, 3476900958, 3144858924, 799089809,
            2899882887, 413231425, 2515242049, 142267098, 1727945779, 3421601015,
        ],
    );
}

#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_limbs_mul_greater_to_out_toom_63() {
    let test = |xs: Vec<Limb>, ys: Vec<Limb>, out_before: Vec<Limb>, out_after| {
        let mut out = out_before.to_vec();
        limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys);
        assert_eq!(out, out_after);
        let mut out = out_before.to_vec();
        let mut scratch = vec![0; limbs_mul_greater_to_out_toom_63_scratch_len(xs.len(), ys.len())];
        limbs_mul_greater_to_out_toom_63(&mut out, &xs, &ys, &mut scratch);
        assert_eq!(out, out_after);
    };
    test(
        vec![
            6746486103788831552,
            2922469023463657485,
            7190781201699911122,
            6369274278675525514,
            11602031538822447399,
            18146097755068799938,
            10715195159596301824,
            1582667531232164822,
            17310503547119278200,
            11108448614311336701,
            16131384432757080248,
            10724146198461527790,
            17486718158725257827,
            6011711000953739951,
            12044019786490872751,
            12126819472937875768,
            11736689834584491812,
            2624631955548590096,
        ],
        vec![
            8718882040837103283,
            12513261442998616191,
            3363599670593686195,
            2576001491054566526,
            8476413363242630098,
            11800520882738943180,
            15256756628116724015,
            15102633230716809194,
            4752404995807312312,
        ],
        vec![10; 27],
        vec![
            11055708298853713344,
            11718134630995530406,
            1540454672309197922,
            2461234873920328802,
            12156343925049526190,
            7669775936281025739,
            5569544286309952271,
            1251802631971472159,
            7852335389754101252,
            16331287242162052217,
            16922468211499817236,
            1090055930057904858,
            4774304109866833132,
            2115064630415334045,
            3041714142401192073,
            5249251501654981623,
            6324653539847586925,
            7895228639492924348,
            13455067205957702368,
            1142009976612635724,
            13095096323291438869,
            4348574203955863428,
            8491467291307697179,
            3535832683825156722,
            3832291870552829557,
            16965222076837711040,
            676179707804463061,
        ],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_63_fail_1() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_63_scratch_len(1, 1)];
    let mut out = vec![10; 4];
    limbs_mul_greater_to_out_toom_63(&mut out, &[6], &[1], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_63_fail_2() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_63_scratch_len(17, 18)];
    let mut out = vec![10; 13];
    let xs = series(3, 17);
    let ys = series(3, 18);
    limbs_mul_greater_to_out_toom_63(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_63_fail_3() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_63_scratch_len(17, 8)];
    let mut out = vec![10; 25];
    let xs = series(3, 17);
    let ys = series(3, 8);
    limbs_mul_greater_to_out_toom_63(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_63_fail_4() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_63_scratch_len(16, 9)];
    let mut out = vec![10; 25];
    let xs = series(3, 17);
    let ys = series(3, 9);
    limbs_mul_greater_to_out_toom_63(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_63_fail_5() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_63_scratch_len(17, 9)];
    let mut out = vec![10; 25];
    let xs = series(3, 17);
    let ys = series(3, 9);
    limbs_mul_greater_to_out_toom_63(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_63_fail_6() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_63_scratch_len(17, 0)];
    let mut out = vec![10; 6];
    let xs = series(3, 17);
    limbs_mul_greater_to_out_toom_63(&mut out, &xs, &[], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mul_greater_to_out_toom_6h() {
    let test = |xs: Vec<Limb>, ys: Vec<Limb>, out_before: Vec<Limb>| {
        let mut out = out_before.to_vec();
        limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys);
        let out_after = out;
        let mut out = out_before.to_vec();
        let mut scratch = vec![0; limbs_mul_greater_to_out_toom_6h_scratch_len(xs.len(), ys.len())];
        limbs_mul_greater_to_out_toom_6h(&mut out, &xs, &ys, &mut scratch);
        assert_eq!(out, out_after);
    };
    // - xs_len * LIMIT_DENOMINATOR < LIMIT_NUMERATOR * ys_len
    // - degree.odd() in limbs_mul_toom_evaluate_poly_in_2pow_neg_and_neg_2pow_neg
    // - degree > 3 in limbs_mul_toom_evaluate_poly_in_2pow_neg_and_neg_2pow_neg
    // - !neg in limbs_mul_toom_evaluate_poly_in_2pow_neg_and_neg_2pow_neg
    // - q != 3
    // - !half in limbs_mul_toom_interpolate_12points
    test(series(2, 42), series(3, 42), vec![10; 84]);
    test(vec![0; 43], vec![0; 42], vec![10; 85]);
    let xs = vec![
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ];
    let ys = vec![
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ];
    let out_len = xs.len() + ys.len();
    // - v_2pow_neg_neg in limbs_mul_toom_evaluate_poly_in_2pow_neg_and_neg_2pow_neg
    test(xs, ys, vec![10; out_len]);
    let xs = vec![
        2176876728, 2496909862, 111654638, 4071443844, 1244732003, 1399710541, 3492272815,
        2804216879, 294683567, 2823495183, 1539340600, 2732661048, 2371405604, 611094747,
        2426195984, 3948451542, 3575143460, 2163084716, 2877537071, 1849282685, 1662381818,
        2022577840, 552741512, 1863034519, 2109621858, 3426780715, 233006082, 2766239663,
        1257764921, 1179443268, 3311729910, 4228711990, 3676801557, 83336617, 52963853, 1461131367,
        615175494, 2376138249, 1373985035, 3055102427, 1823691121, 175073115, 3051957217,
    ];
    let ys = vec![
        344785207, 1075768263, 3315797254, 2656376324, 160336834, 3872758991, 671370872,
        1253701757, 217686653, 4064957864, 1185854346, 2308111201, 847669579, 195002426,
        1955159211, 2003106801, 1041767923, 3605273739, 3153084777, 2806535311, 1401436525,
        1148858479, 958627821, 1267879008, 4138398998, 1028065582, 3914213477, 3370118288,
        4054975453, 1815994585, 2486521917, 995353494, 16609723, 4010498224, 1214270934, 797624362,
        4000265982, 1287753121, 874311717, 2200865401, 21122981, 1507911002,
    ];
    let out_len = xs.len() + ys.len();
    // - r4last.leading_zeros() < 3 in limbs_mul_toom_interpolate_12points
    test(xs, ys, vec![10; out_len]);
    let xs = vec![
        2327202328, 3179332026, 2188958336, 2717879675, 130062885, 140536268, 2499125438,
        3163111280, 4259661702, 2176278885, 422519228, 2482586299, 2904549185, 656169575,
        2052350629, 1346745024, 2132509288, 3672720658, 1036389958, 1864007789, 4247227128,
        3920036168, 1436562554, 4261984498, 3509215437, 583752676, 3145348403, 2267709494,
        2846186667, 95392897, 3743233716, 2210401890, 333864866, 4114644153, 3030283850,
        2885600773, 209380485, 753945396, 719327396, 1293498320, 881901364, 2799735404, 3880748109,
        2227099476, 2045911493, 279042015, 1825819541, 1783146691, 2256898093, 2186071881,
    ];
    let ys = vec![
        4062960470, 3852836537, 2696572187, 2332897564, 3819654112, 1805852435, 2339319161,
        3891614436, 3143079880, 3244604349, 2122448594, 1926396564, 3938383812, 51745369,
        2731805677, 4257919711, 2550692774, 4079294279, 223709465, 1648526554, 689775843,
        3524108772, 1404538310, 806199241, 4278266886, 2467028886, 3773289773, 3246095241,
        2201055218, 2036154035, 3144210007, 423367788, 3883829868, 2190252193, 2069131777,
        3027047320, 1576225469, 3459606326, 2343356582, 2658410138, 1927376994, 3129832669,
        3772482523,
    ];
    let out_len = xs.len() + ys.len();
    // - xs_len * LIMIT_DENOMINATOR >= LIMIT_NUMERATOR * ys_len
    // - xs_len * 5 * LIMIT_NUMERATOR < LIMIT_DENOMINATOR * 7 * ys_len
    // - half
    // - degree.even() in limbs_mul_toom_evaluate_poly_in_2pow_neg_and_neg_2pow_neg
    // - degree > 5 in limbs_mul_toom_evaluate_poly_in_1and_neg_1
    // - s <= t
    // - half in limbs_mul_toom_interpolate_12points
    test(xs, ys, vec![10; out_len]);
    let xs = vec![
        1940830933, 3780770129, 1587254032, 832573251, 1504418072, 4247592896, 317874907,
        949850421, 2252881736, 3574316069, 3062236166, 1396410954, 3249498785, 3495392204,
        540855070, 1908700137, 1469179505, 4199276220, 953657385, 3056452157, 2141569526,
        2342475731, 3746376146, 3271677606, 2770490239, 2212992129, 1758619376, 1446549455,
        409094501, 767129031, 3284625381, 1887741449, 1134874072, 2988924415, 1641550007,
        856704035, 80648349, 1467185629, 2753807208, 1609415681, 4087676277, 3276525355,
        1530490532, 3475014952, 1971819359, 2190766950, 2667577576, 2404497182, 4128259693,
        2449514447, 4199089872, 2205116036, 4089987616, 457231895, 2931469481, 3147651033,
        2352907189,
    ];
    let ys = vec![
        3461606200, 1584050797, 14355481, 3385840230, 1703326352, 1625259628, 3642322228,
        911402341, 2158835226, 939248485, 3607511108, 2863853568, 1611642161, 1312857772,
        1839433327, 567060478, 3139863681, 3642698184, 3744632443, 712538472, 2692932947,
        576185818, 156934113, 518107105, 2803035863, 2284220097, 3447382922, 2400125006,
        3565062840, 160044186, 3644393084, 4196433258, 3391883838, 1115703759, 2380388002,
        962895870, 4001772616, 2311278419, 2620271020, 3047789793, 3229254302, 3182628087,
        2718480927, 2872538422,
    ];
    let out_len = xs.len() + ys.len();
    // - t < 1
    test(xs, ys, vec![10; out_len]);
    let xs = vec![
        3796896013, 4144448610, 2569252563, 2859304641, 1027973602, 3158196152, 4058699545,
        2002924383, 3295505824, 695758308, 544681384, 3452307839, 1190734708, 4232023153,
        451772934, 673919865, 2022672425, 3493426012, 1142609332, 477542383, 1304798841, 461115870,
        3268103575, 2243523508, 606810814, 4235312469, 1885993181, 114475077, 757688489,
        1965769398, 260629125, 2265559181, 2568323569, 4202738507, 422918034, 1258453131,
        3552221985, 1666914845, 4063631552, 1893061685, 1362616670, 3828572660, 3003680479,
        119501228, 2101943449, 1119123129, 2512417484,
    ];
    let ys = vec![
        610160726, 3751120540, 2655318738, 2490069121, 732352936, 1985503906, 765573690,
        2709177647, 3058016350, 1432725430, 2213840145, 1911049343, 3116245242, 519557432,
        1828983405, 3092431113, 3844759473, 547304293, 1609305183, 1824076406, 2409386071,
        2970173039, 4255413180, 894750419, 90356879, 2880999631, 2157180976, 2261258057, 715581698,
        332174009, 27958638, 2464799420, 3232925197, 1952944696, 915312443, 1464711675, 4079172443,
        2445511584, 2092009263, 3412361485, 2354390078, 3106038172, 3481973486,
    ];
    let out_len = xs.len() + ys.len();
    // - s < 1
    test(xs, ys, vec![10; out_len]);
    let xs = vec![
        2187046946, 3600373521, 4275090943, 2120016813, 4177241875, 3185774231, 2397692077,
        1015362399, 2178889151, 3433916223, 1688082118, 1971242178, 236388706, 3802829765,
        521309115, 2299816689, 3207614143, 1053195464, 3584561145, 1178690670, 2940812254,
        3321982035, 2754825123, 3073598062, 202404806, 547895545, 1188944547, 1056841779,
        529463805, 204665384, 850370055, 2063320161, 3724100092, 1180272690, 1398467003,
        2814052449, 1311768018, 659771105, 3226477227, 4230080238, 2134344405, 1461172705,
        2728018383, 1816821358, 3231137250, 2012377728, 2206916761, 3121807673,
    ];
    let ys = vec![
        1717557648, 1819215517, 3449795284, 844168976, 1574237607, 758725457, 762624299, 533122182,
        1201164787, 1968174784, 896982568, 3419630169, 2247559545, 3983311870, 3975342941,
        1112833399, 2721518545, 2493587613, 3444837338, 3313000598, 751186769, 2970698395,
        915811688, 1206259449, 1340427760, 3844346545, 3762393860, 543253569, 1197933603,
        3734607133, 4037352821, 2263945478, 2831362781, 3363558852, 476952769, 1916745391,
        208671986, 2395250976, 1549715018, 2746690542, 1219103496, 256305249,
    ];
    let out_len = xs.len() + ys.len();
    // - s_plus_t > n in limbs_mul_toom_interpolate_12points
    test(xs, ys, vec![10; out_len]);
    let xs = vec![
        1976230069, 2821313230, 4002048052, 2248747478, 1208640865, 1469538686, 2438066233,
        1106183979, 1877645648, 2583513281, 904899723, 1001323826, 3134049747, 292171929,
        1479818350, 821125410, 2017124898, 3447449059, 2073983663, 1214861045, 3270809855,
        2826108666, 412311360, 3687943078, 157663911, 447468817, 1727023746, 1120132848, 462566659,
        21711861, 2204912119, 631663514, 2655508903, 2912870262, 1326931248, 1409724492,
        3912444286, 1986726296, 190162730, 675575771, 234714100, 3787240294, 3149710501,
        1950469069, 1222949463, 218525862, 929916299, 1757577031, 3896857869, 443052809,
        4256330379, 1106528307, 2502814887, 108409846,
    ];
    let ys = vec![
        3774873792, 2622161570, 566787739, 1447674683, 1128900692, 2570098345, 3920242059,
        2431899603, 1456341665, 269610676, 673205188, 3712878022, 3795578329, 996518376,
        3414916195, 4167667588, 4013410429, 724257700, 698186720, 1170923258, 3652768880,
        1373260172, 3271469225, 971070649, 1556038273, 2204702414, 673789949, 3790414001,
        1550521405, 2173912108, 70968354, 1856452807, 2648613270, 2751500372, 1057118618,
        3117394831, 4409774, 2422780755, 3367234488, 1080583495, 29356841, 3627216363,
    ];
    let out_len = xs.len() + ys.len();
    // - s > t
    test(xs, ys, vec![10; out_len]);
    let xs = vec![
        2764481948, 3824853452, 3714446166, 1652416239, 2448871004, 3349954116, 2715554665,
        2953094534, 2191528165, 1105735060, 407641991, 1058849514, 2583237649, 3635224830,
        1509496009, 2360185935, 2419261549, 2433663350, 262632960, 3504095388, 2570319009,
        2415092334, 72373859, 3953007752, 3259518037, 3401184350, 574975346, 1921349734,
        1293058836, 2824387015, 670301824, 3449438821, 3149566748, 2370941125, 3445476733,
        1172535390, 684380840, 4007537582, 3019960994, 3833788436, 2407231528, 532343833,
        438092212, 830534904, 325324494, 1629611634, 3991887007, 1617691624, 3806774950,
        2737609900, 4123817599, 1139254855, 4270594452, 3772632696, 357643096, 978439292,
        3535266500, 1036728326, 408519941, 386395864, 986295007,
    ];
    let ys = vec![
        2893157767, 2933782072, 1630695663, 765017133, 148924741, 3933388144, 2827967305,
        1580462312, 4233997190, 2184167709, 1124313531, 1269787970, 2637050113, 1899399034,
        458443927, 676372848, 3341236235, 2358837775, 78253712, 1308766267, 1398616295, 442007911,
        3803960772, 2890078708, 2362278228, 452577827, 2295445770, 1281833658, 3733263779,
        3192119570, 1465309963, 4149236735, 2550067398, 3391554453, 3763654782, 280954439,
        4216404337, 2988297132, 1171366979, 752568358, 3832355781, 3002295862,
    ];
    let out_len = xs.len() + ys.len();
    // - xs_len * 5 * LIMIT_DENOMINATOR < LIMIT_NUMERATOR * 7 * ys_len
    test(xs, ys, vec![10; out_len]);
    let xs = vec![
        1537074800, 903591185, 3505885895, 1600301704, 2247503777, 2456507858, 354178772,
        4264234279, 4276311343, 2137271746, 3095634214, 3503644667, 3271712752, 1235289576,
        3972513632, 4268165027, 3304957815, 2349877036, 1814187379, 1622480961, 1887152020,
        617829740, 2759792107, 2650325546, 3834300382, 1711067002, 16368281, 3248020475,
        1355293366, 2500355734, 3216660200, 2844209744, 919471841, 2536405197, 286948869,
        3207728956, 1786641001, 3909697676, 2990524533, 3373134471, 2770917041, 2941741335,
        2275165617, 610985518, 1663622513, 780492488, 696913656, 1787332447, 1693914179,
        2059746330, 4084862137, 1720114882, 2072770321, 2800094080, 164377327, 114079185,
        1630830573, 866212705, 86571916, 2701570437, 1022361296, 2774191689, 1485998454,
        1449541799,
    ];
    let ys = vec![
        10887125, 840662268, 2350057862, 3489480809, 2643647461, 2120151555, 433525765, 1719122308,
        3784715068, 3156307967, 4113669583, 607844816, 2149779595, 55766995, 3922134877,
        1464452041, 2877070520, 3517698059, 3219767758, 138329276, 1434547315, 1010269423,
        3836852303, 521525549, 1124005096, 128173038, 1627976147, 4217098680, 963901397,
        4003948876, 4078383999, 3163439869, 1376461045, 1260808800, 1583549957, 3016546386,
        601137572, 2476346948, 1057124592, 2232232546, 2939285402, 2703166574, 2566511508,
    ];
    let out_len = xs.len() + ys.len();
    // - xs_len * LIMIT_NUMERATOR < LIMIT_DENOMINATOR * 2 * ys_len
    test(xs, ys, vec![10; out_len]);
    let xs = vec![
        2480817744, 2385986715, 908796583, 3725142486, 4259996640, 2324291843, 2514777689,
        776517112, 1179390166, 2884250121, 2107025487, 1847592315, 1214792717, 581761941,
        2035752941, 3257884740, 1011095107, 388625485, 621566511, 1878249130, 2298430809,
        3893830507, 2516166455, 1685998768, 3349147146, 4262358486, 164529678, 1000098113,
        1468664761, 1088142633, 2140348214, 672483433, 4236152545, 460911546, 1948312076,
        1030937440, 3633681142, 1170002101, 2159285228, 1104198886, 1581288546, 2266152509,
        1437951300, 3854459332, 88193405, 3804599756, 577997778, 3610194716, 2527782134,
        4194448103, 3390832927, 863423772, 2308481008, 1764994151, 2876150765, 474256942,
        3850214133, 2831691105, 4251752821, 80285354, 3225163007, 84390462, 1489215151, 1516077116,
        299402893, 1093360002, 706962212, 375054336, 678692965, 2794629958, 3684518009, 1067098399,
        3918266067, 770155119, 1400555696, 4260143847, 3420662760, 2234352998, 2627202272,
        2396298990, 2703934662, 2975030448, 1678542783, 3962857080, 2037990778, 2350341680,
        3690768614, 3327392397, 2374080995, 1568940040,
    ];
    let ys = vec![
        2432887163, 3617411153, 4065664491, 954897002, 1958352130, 2690853400, 3170435422,
        333223996, 1886503369, 2874118364, 2360990628, 3409169651, 14803166, 2428352279,
        2882529293, 215157778, 3595826381, 1351666697, 3213081864, 1796627015, 138520647,
        1446708749, 549025603, 1154696063, 951257454, 1061151557, 3578338019, 553024835,
        1032056788, 3332695385, 1916952270, 1402847201, 418140204, 1113800470, 3311963507,
        3579825680, 283695808, 1030062334, 2885288472, 2307021635, 1215165167, 361703549,
        3359666682, 2960119991, 3759575408,
    ];
    let out_len = xs.len() + ys.len();
    // - xs_len * LIMIT_DENOMINATOR < LIMIT_NUMERATOR * 2 * ys_len
    // - q == 3
    test(xs, ys, vec![10; out_len]);
    let xs = vec![
        2182584668, 370736031, 944597706, 368333101, 3076089385, 4269551750, 455799119, 1640998687,
        1332255273, 3039440200, 1094187469, 4158542740, 4241437189, 786279542, 3313987323,
        801901648, 2460914857, 2458651362, 1161118074, 3733983107, 1911753349, 4261306583,
        981590361, 1357088215, 210591422, 1159943023, 510963968, 2705428227, 3460159465,
        1967595187, 703584117, 3474024702, 3343010520, 1232104952, 823854220, 4012290690,
        3252492197, 3975386640, 1309751464, 232265040, 2026518879, 794539121, 1849747498,
        773993567, 2415934846, 842827728, 25297943, 3952540535, 2909076393, 4183158950, 2579267900,
        898983053, 2480815324, 1004385686, 3272214418, 2360496610, 3884948711, 3937994494,
        1355835525, 1862072763, 4077270583, 456721854, 1202741767, 1334238573, 3202598432,
        2518498766, 1873498914, 1155219866, 3257357513, 3381800028, 777225471, 1628571355,
        281982096, 1238331533, 728101793, 407378640, 1088081860, 2405377044, 2080950804,
        3105324348, 3065313268, 2776290680, 1200951260, 1789619269, 1088225065, 317598486,
        924903972, 3504476787, 1605816151, 388266283, 1613602905, 4051481387, 2773856406,
        3434866445, 2039264971, 1587433780, 1787644933, 2852323335,
    ];
    let ys = vec![
        3040086267, 3720432305, 3025753876, 3307555779, 2232302878, 1705545587, 3746861739,
        3551552480, 3791909589, 3559707401, 3597994914, 1201195479, 2759785652, 2538497144,
        2628719068, 1220743906, 2592330951, 357425155, 2683446134, 369894528, 2918070813,
        3201581079, 352827384, 2667389301, 406071886, 1478662115, 3424718337, 3498162517,
        1851891341, 2009161130, 4175528772, 2739823403, 2691610015, 530787751, 2995441702,
        238468207, 84087963, 2802633771, 2722772179, 1905704311, 791349630, 4036308669, 1333503772,
    ];
    let out_len = xs.len() + ys.len();
    // - p == 9, q == 4
    test(xs, ys, vec![10; out_len]);
}

#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_limbs_mul_greater_to_out_toom_6h() {
    let test = |xs: Vec<Limb>, ys: Vec<Limb>, out_before: Vec<Limb>| {
        let mut out = out_before.to_vec();
        limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys);
        let out_after = out;
        let mut out = out_before.to_vec();
        let mut scratch = vec![0; limbs_mul_greater_to_out_toom_6h_scratch_len(xs.len(), ys.len())];
        limbs_mul_greater_to_out_toom_6h(&mut out, &xs, &ys, &mut scratch);
        assert_eq!(out, out_after);
    };
    test(series(2, 42), series(3, 42), vec![10; 84]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_6h_fail_1() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_6h_scratch_len(1, 1)];
    let mut out = vec![10; 4];
    limbs_mul_greater_to_out_toom_6h(&mut out, &[6], &[1], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_6h_fail_2() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_6h_scratch_len(41, 42)];
    let mut out = vec![10; 83];
    let xs = series(3, 41);
    let ys = series(3, 42);
    limbs_mul_greater_to_out_toom_6h(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_6h_fail_3() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_6h_scratch_len(42, 41)];
    let mut out = vec![10; 83];
    let xs = series(3, 42);
    let ys = series(3, 41);
    limbs_mul_greater_to_out_toom_6h(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_6h_fail_4() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_6h_scratch_len(41, 41)];
    let mut out = vec![10; 82];
    let xs = series(3, 41);
    let ys = series(3, 41);
    limbs_mul_greater_to_out_toom_6h(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_6h_fail_5() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_6h_scratch_len(42, 42)];
    let mut out = vec![10; 83];
    let xs = series(3, 42);
    let ys = series(3, 42);
    limbs_mul_greater_to_out_toom_6h(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_6h_fail_6() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_6h_scratch_len(42, 0)];
    let mut out = vec![10; 42];
    let xs = series(3, 42);
    limbs_mul_greater_to_out_toom_6h(&mut out, &xs, &[], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mul_greater_to_out_toom_8h() {
    let test = |xs: Vec<Limb>, ys: Vec<Limb>, out_before: Vec<Limb>| {
        let mut out = out_before.to_vec();
        limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys);
        let out_after = out;
        let mut out = out_before.to_vec();
        let mut scratch = vec![0; limbs_mul_greater_to_out_toom_8h_scratch_len(xs.len(), ys.len())];
        limbs_mul_greater_to_out_toom_8h(&mut out, &xs, &ys, &mut scratch);
        assert_eq!(out, out_after);
    };
    // - xs_len == ys_len || xs_len * (TOOM_8H_LIMIT_DENOMINATOR >> 1) < TOOM_8H_LIMIT_NUMERATOR *
    //   (ys_len >> 1)
    // - !(Limb::WIDTH > 36 && q == 3)
    // - r6last.leading_zeros() < 3 in limbs_mul_toom_interpolate_16points
    // - !half in limbs_mul_toom_interpolate_16points
    test(series(2, 86), series(3, 86), vec![10; 172]);
    let xs = vec![
        3581553119, 2147449432, 208926434, 2037430803, 4143975728, 2356343321, 937192435,
        1637432038, 661638621, 1801480924, 3779152128, 4243491821, 1667774376, 1715755489,
        3661813139, 1605971891, 4030695606, 2961165054, 1368430397, 2222904896, 2817587025,
        1714442303, 3822714979, 300305701, 1874484285, 2601340412, 2275789197, 2695461089,
        2246464394, 1119579754, 1646098622, 3280004748, 33497272, 1940830933, 3780770129,
        1587254032, 832573251, 1504418072, 4247592896, 317874907, 949850421, 2252881736,
        3574316069, 3062236166, 1396410954, 3249498785, 3495392204, 540855070, 1908700137,
        1469179505, 4199276220, 953657385, 3056452157, 2141569526, 2342475731, 3746376146,
        3271677606, 2770490239, 2212992129, 1758619376, 1446549455, 409094501, 767129031,
        3284625381, 1887741449, 1134874072, 2988924415, 1641550007, 856704035, 80648349,
        1467185629, 2753807208, 1609415681, 4087676277, 3276525355, 1530490532, 3475014952,
        1971819359, 2190766950, 2667577576, 2404497182, 4128259693, 2449514447, 4199089872,
        2205116036, 4089987616, 457231895,
    ];
    let ys = vec![
        1495737173, 3863569894, 2781409865, 2031883388, 2335263853, 2715800358, 580338429,
        3465089273, 419683969, 372309798, 2092398197, 1587236508, 1706866472, 1926863329,
        2427550983, 3014840641, 2591183237, 311998012, 1838159904, 2382380991, 3168560843,
        2457672651, 1329938456, 1585986499, 32624746, 1886190156, 1819802220, 4189456784,
        2354442118, 1007664036, 3528608675, 3607011918, 3175583218, 2103466232, 4139172560,
        1977990249, 408055457, 1917901811, 4285926188, 2576630504, 3833124229, 664620480,
        3594197860, 38119241, 2843152292, 1589895470, 132829200, 911163756, 3350029197, 141124331,
        628197809, 3184483823, 2738720089, 3684675439, 2998575143, 2394913714, 2088681890,
        2743885961, 2257026807, 2812703572, 678096205, 2964972038, 1641032123, 3238217254,
        2452280240, 193873172, 277301379, 106064560, 2264572378, 3461606200, 1584050797, 14355481,
        3385840230, 1703326352, 1625259628, 3642322228, 911402341, 2158835226, 939248485,
        3607511108, 2863853568, 1611642161, 1312857772, 1839433327, 567060478, 3139863681,
    ];
    let out_len = xs.len() + ys.len();
    // - r5last.leading_zeros() < 7 in limbs_mul_toom_interpolate_16points
    test(xs, ys, vec![10; out_len]);
    let xs = vec![
        3998843185, 3237409891, 364765898, 887299373, 875693912, 790653310, 1949338310, 309040598,
        2753929769, 1560315881, 2158749638, 124625299, 1949071109, 4293842935, 3418183766,
        1387429696, 64843603, 1303399904, 455978049, 3724928213, 4182321093, 1342619213,
        1692503310, 2594578249, 2811338438, 1715625698, 751013184, 1529801113, 2582454920,
        4199343251, 3183268625, 2516721877, 1167772050, 2317983168, 1793272983, 311653702,
        3588179354, 661601476, 2154410870, 2334965650, 4135084105, 1682699224, 47903600,
        3273743199, 3845966395, 1357302998, 3756718174, 2451701689, 2321438159, 3211448326,
        2377823945, 50814995, 1672303030, 4158805623, 2661886690, 1846253587, 702414278,
        4059841129, 3727323213, 1424047747, 2939622087, 2231052374, 2013876172, 2053003398,
        1741887596, 3509712959, 5142212, 3825464748, 3375048072, 338658021, 2655991044, 2889153792,
        2332483687, 934832926, 3863652984, 1414099507, 2895368376, 1013122176, 2794762768,
        2981493251, 3152252275, 1564424419, 536147906, 242465174, 3000707896, 3526733161,
        943706939, 349997931, 1497577916, 3473622068, 1517005385, 2206423568, 1544165865,
        3199998353,
    ];
    let ys = vec![
        1562512360, 3239315566, 2225439589, 502536858, 1867965636, 618137922, 4149231651,
        476678563, 4203415530, 4178036608, 1956783646, 4023049148, 2645084690, 270122366,
        201340005, 4276855303, 1021151730, 916821881, 663141922, 2795604136, 3762385264, 348487994,
        2655354829, 343872945, 2028904125, 1525417887, 867188532, 3911999830, 2139706847,
        3256484706, 961423019, 1530068826, 3577946967, 2361035355, 337639742, 3774308229,
        2185652798, 3532716804, 4018761888, 1357817255, 2216301712, 2861241181, 3053055924,
        3777579308, 795689292, 3386662598, 4160296368, 2005833155, 1297354264, 2851045342,
        954306552, 1613754854, 2227385445, 528669733, 3315107199, 3402866739, 1117279433,
        232818134, 1490857876, 1962534623, 1227821174, 159891958, 1385848424, 4061426539,
        647828819, 2061390815, 4239314784, 1854131914, 3258304017, 524974854, 450125309, 684998491,
        2942294237, 4191667771, 2230185588, 1844054665, 193300986, 2652500966, 4050934267,
        1133780381, 3709046706, 909867408, 4209959016, 4275912160, 277155368, 1775051743,
        4190065677,
    ];
    let out_len = xs.len() + ys.len();
    // - !(an == bn || an * (TOOM_8H_LIMIT_DENOMINATOR >> 1) < TOOM_8H_LIMIT_NUMERATOR * (bn >> 1))
    // - xs_len * 13 < 16 * ys_len
    // - half
    // - s <= t
    // - half in limbs_mul_toom_interpolate_16points
    // - s_plus_t > n in limbs_mul_toom_interpolate_16points
    test(xs, ys, vec![10; out_len]);
    let xs = vec![
        2456061149, 2562918666, 2903450513, 1994190773, 99234624, 3722083920, 4262323306,
        202219441, 4201857695, 3988878636, 1533308334, 401400520, 1069756554, 2457773969,
        2892388936, 3423117995, 1944069442, 492036629, 3426800580, 2282483359, 4006366620,
        1695364515, 2555180845, 1669287836, 349290429, 778467450, 2020203604, 2218159817,
        1450404019, 1278304750, 2412695340, 1592154884, 3868182043, 2240370481, 3859902860,
        1008825116, 412233394, 2475457637, 3664379433, 4204584226, 2750684469, 4113507475,
        2916584959, 285955744, 739598569, 18278051, 3768126932, 2181905109, 2612988076, 1827656088,
        1160380415, 4160443718, 1846086671, 3050604645, 2547108010, 2828666778, 3252702258,
        3885923576, 2331974758, 730724707, 1528859315, 4288784328, 3677151116, 445199233,
        3304488688, 3566979465, 3541025426, 2491779846, 3112990742, 2583249486, 3403111749,
        1930721237, 3428792463, 2896462048, 2985885576, 1819460734, 21206096, 3560441846,
        987100555, 2844904275, 84854892, 1268249628, 3963306788, 3338670067, 2504599089, 65588657,
        321493327, 4249673617, 4150876068, 721566898, 2186945060, 922948272, 1502464627,
        1426914435, 2906888275, 3454987739, 2609132626, 2073366782, 1058809001, 1226951003,
        2624503637,
    ];
    let ys = vec![
        3941840558, 1662743930, 1905993615, 2485835810, 3925643251, 3071436009, 851721712,
        1325046168, 3214018378, 1465803515, 2459667310, 2361559987, 2668552637, 2425633974,
        3200812339, 2594448814, 4170435967, 1112582678, 3198704424, 4028094030, 2482710119,
        2990475705, 708195759, 612294539, 2794828841, 2498141427, 3805184114, 3010938369,
        1479667740, 660767380, 1641177565, 1782849661, 1915222559, 1626388136, 1816788637,
        1338361170, 783877621, 4003339370, 1930607900, 1259399167, 3351643097, 1641708262,
        967800396, 1800752717, 2198926109, 1163817943, 2710351254, 451351637, 1285647338,
        865168955, 645286276, 2685132510, 1773153387, 4273868103, 2604563645, 4105767904,
        2556376985, 158907213, 3579937882, 3059825408, 1920542835, 528717490, 1430681949,
        616489338, 597761261, 3760865497, 963173252, 2915089223, 1441674715, 1717557648,
        1819215517, 3449795284, 844168976, 1574237607, 758725457, 762624299, 533122182, 1201164787,
        1968174784, 896982568, 3419630169, 2247559545, 3983311870, 3975342941, 1112833399,
        2721518545, 2493587613, 3444837338,
    ];
    let out_len = xs.len() + ys.len();
    // - s > t
    // - s_plus_t <= n in limbs_mul_toom_interpolate_16points
    test(xs, ys, vec![10; out_len]);
    let xs = vec![
        2166912886, 3021127478, 1088026295, 863338925, 1902617744, 2706401163, 3211745137,
        3537828549, 2310228205, 2585051285, 3210490216, 612524924, 269492174, 83675252, 3088638931,
        2020592214, 884676247, 2114372012, 2448236682, 3651962645, 4142890271, 3807368959,
        3038213130, 1740849525, 1839016815, 3718350068, 1798083657, 4018300117, 2557824626,
        1367910868, 3524299249, 2718734101, 2199735437, 2156117642, 3314330151, 91570504,
        1763771190, 730175380, 3035959105, 930897603, 4104577491, 1545111417, 2973200358,
        1531233892, 3216274102, 2879326700, 4043195388, 4012932329, 1225928231, 3148638781,
        3350412374, 571148440, 42117077, 2619230436, 570695610, 3533920410, 2337569860, 2616128436,
        1101128308, 986097032, 4127211776, 1459526104, 121723950, 1459838938, 1563443987,
        3106615121, 2637954840, 238917822, 3086105506, 2960421944, 2937286162, 3871313970,
        554575295, 450448609, 493464699, 3492897008, 3198787067, 2691863142, 874317820, 1804414164,
        572281701, 2867423932, 412542374, 239109523, 4270925097, 1858402222, 3784404338, 162014339,
        182208178, 171269941, 1556499146, 3122050585, 2070559038, 1293272336,
    ];
    let ys = vec![
        131674806, 603734923, 2440163395, 2896151903, 2142986136, 3702794463, 407655836,
        1281722924, 1990690788, 2883417209, 1106804242, 965105623, 3369860750, 2422075060,
        1042530548, 1864787458, 1722387953, 324177444, 3169639558, 1324636283, 1394919591,
        1382200609, 4014256585, 1943865290, 1318181231, 2753206532, 465681637, 3556126827,
        3726586809, 2859198026, 1880611700, 2743775719, 2312093882, 2611444395, 2043850780,
        1748249887, 1827465861, 1827026074, 3842470222, 886015214, 1202152837, 1760966154,
        1303682364, 2141063912, 2027419958, 3046273896, 276337299, 1629565318, 3973822671,
        3586055166, 515343743, 4150823547, 3812419028, 4047886683, 408756427, 30807697, 3839670586,
        3241113948, 1946580966, 211283947, 1648787704, 1254977229, 324210665, 409019127, 999906525,
        3589880779, 2652719468, 2740912614, 75319316, 3276454084, 3598090610, 225502084,
        1039377126, 3755265351, 299690912, 2582901309, 891564570, 1062813956, 318910996,
        2153235228, 2834278326, 130377847, 977327805, 3290994684, 2956083989, 826986477,
        1417957671, 2007397536, 3845476521,
    ];
    let out_len = xs.len() + ys.len();
    // - s < 1
    test(xs, ys, vec![10; out_len]);
    let xs = vec![
        1012246656, 3781649075, 2144318856, 2608903399, 688555306, 1040133166, 3831584367,
        1593112617, 1823654254, 840638304, 3109717334, 188514461, 398797195, 75875990, 1486449995,
        4269057266, 3729965858, 1861862237, 3631015569, 3651675458, 103019792, 4115125912,
        854107191, 437995362, 1626634580, 1556708150, 2197935825, 142443256, 2516681044, 165384798,
        622726627, 2804275513, 3768014324, 1019999140, 1630141384, 1569491385, 2650112147,
        404117490, 959368136, 1567892691, 3740061638, 1492035182, 2806958299, 3558543973,
        2394278513, 193040368, 140963621, 2363913022, 521760299, 1509309827, 1222622424, 236238235,
        148145098, 1185145642, 4050835140, 3496710859, 2912031916, 2811044753, 293786270,
        1593967022, 3059741198, 957447590, 999733770, 3225819121, 389969264, 1617194653, 930042654,
        2073424372, 1334988223, 2244143480, 3036433790, 314486992, 3505856530, 2253001666,
        2732695676, 2150239253, 2058771616, 2553846568, 3156714961, 275374496, 2154639432,
        1705499511, 2661128488, 2996751598, 1991220721, 2971546013, 947096109, 1988630082,
        3629027637, 2894867708, 982953971, 1288656915, 3544920961, 2725968940, 2718109332,
        1685012966, 2463009759, 1861144639, 2364403606, 3459863283, 983775524, 3466796660,
        1976698215, 708098181, 3069387825, 3638611575, 2579187312, 632774203,
    ];
    let ys = vec![
        1809516468, 2803977220, 3078159083, 486681337, 1568336896, 4117841648, 422990983,
        2706208156, 3747890395, 2705136812, 2904348475, 1582408791, 723059442, 3021061511,
        4080366324, 344817763, 4291264074, 846996023, 4266039848, 1034099747, 3469554547,
        1098932136, 4197098884, 2840685725, 3598360260, 3858664271, 2988904929, 3788334949,
        2778508367, 2862059554, 3453038230, 315104137, 659918534, 3119028578, 178870393,
        1471088291, 908295683, 5373305, 1643272591, 1306263419, 808966614, 4084169993, 740212697,
        4046005160, 2962244838, 2183688745, 2126344144, 2041407930, 201066579, 4119015900,
        3263668172, 1482349211, 660638692, 596028971, 3002749394, 3127689329, 147925750,
        1069598238, 1868876453, 1293290441, 1391999979, 1064595909, 1912901608, 751720124,
        313663396, 2718231373, 1813378594, 1913592155, 2372166689, 312370283, 1294902637,
        1519106439, 2159217107, 3862662328, 3650935678, 3673744494, 1365354839, 4239084491,
        2676645359, 906655247, 2012326184, 363781147, 121405308, 3179196888, 1415338639, 788446024,
        2165764832,
    ];
    let out_len = xs.len() + ys.len();
    // - Limb::WIDTH <= 9 * 3 || xs_len * (TOOM_8H_LIMIT_DENOMINATOR >> 1) <
    //   (TOOM_8H_LIMIT_NUMERATOR / 7 * 9) * (ys_len >> 1)
    test(xs, ys, vec![10; out_len]);
    let xs = vec![
        4119986492, 3769784140, 1016845783, 1462133596, 4070906664, 3720888633, 4162266585,
        357581522, 1461543577, 4176530320, 4211178471, 3101664977, 3852655570, 166352500,
        1437797012, 3499540684, 1659293446, 4040889056, 2872135683, 3443479700, 655062062,
        1438477128, 1251147166, 2862092792, 1899445621, 1706155530, 2740470033, 732343724,
        3637646944, 4084594941, 2604690616, 4034800391, 3052473123, 2211244267, 947388355,
        584537104, 4143732645, 753960748, 3490638800, 3716731483, 812984705, 1845462359, 65215620,
        4176252687, 2616921776, 2554085123, 4119079055, 4015290385, 697509015, 234073199,
        845662165, 1354305840, 981298174, 1565184955, 207005143, 3409837524, 1220287572, 729153595,
        4103593694, 3696910742, 3965466426, 2266950204, 3856396952, 1764904477, 2684424799,
        2851670593, 1238534904, 1193928568, 775873269, 1360693711, 2015831201, 4011315900,
        3412793575, 214657369, 4288738109, 2288646350, 4016569358, 3132961648, 4045851426,
        3660819126, 4044839853, 3089247133, 2180567261, 2646234732, 1387965746, 2657998851,
        713566741, 3356621670, 3732665499, 1904626236, 64110644, 1408823950, 3590020345,
        2474929782, 849015605, 44073994, 1392682200, 2899713947, 276297197, 2522590522, 3057126922,
        2424068009, 1656987557, 1344629217, 2147192728, 3358875432, 3127883048, 1416207351,
        2542101426, 711240683, 2104649063,
    ];
    let ys = vec![
        2166824272, 3241826034, 3119928903, 4235394337, 702909009, 952063230, 3767289278,
        3471432542, 1289423414, 4165356232, 1144080646, 1098693005, 2158644075, 3466960484,
        107907398, 1849951849, 1697379716, 3245621651, 789557144, 3055443426, 3784862213,
        3687293729, 3527108073, 2085509714, 2098672286, 4237955923, 1799505183, 4280924128,
        1714047371, 679046973, 2920210487, 2630108623, 3799940507, 2820960341, 2480102998,
        3063576036, 1124333889, 3649141414, 3766465016, 1301782752, 3365747207, 318110166,
        1798715740, 3939897237, 1972418626, 525713989, 4204639302, 1845175119, 3066964494,
        3197166778, 2045294098, 1778200774, 1122512884, 487879411, 3912690682, 2631572995,
        119236796, 3659697136, 875446358, 2784882013, 724223194, 2290104863, 3553626657,
        1049986268, 1149074120, 457683007, 342994481, 3969592954, 4124706173, 793289745, 50385201,
        428623925, 330776585, 154172871, 652756593, 1305471058, 3295431270, 1976260297, 1729803474,
        1132360814, 2965768226, 3482945302, 2017386623, 1093051437, 2874103717, 2882475975,
        3735654948, 1766940801, 3723445548, 3203977826, 1788553316,
    ];
    let out_len = xs.len() + ys.len();
    // - t < 1
    test(xs, ys, vec![10; out_len]);
    let xs = vec![
        562392731, 220608607, 3016011233, 1988425644, 1753293069, 202000452, 2988281129,
        1833355482, 2406139229, 3819843447, 3864310556, 2964129037, 3243750205, 1300008578,
        213321522, 4162936161, 3499001762, 2548817881, 797422884, 3464557820, 3172918275,
        3342298017, 4095467160, 1278405537, 2731045246, 1797909329, 915931552, 1234105774,
        1721010619, 393116356, 3595672812, 246921897, 3156619416, 367413315, 835896205, 1133867872,
        732164137, 2864249493, 4191919416, 2012484604, 2046119300, 464214194, 1309621688,
        2133767576, 1817717936, 3210357881, 2703486295, 73128890, 3834854978, 1247202589,
        3867658887, 743571365, 623502109, 2414902368, 4157134303, 505113368, 3563229135,
        2326845431, 1870329856, 412186635, 643126122, 918171482, 3174437348, 992920198, 2549886607,
        2594507263, 870344606, 3354423872, 3768408002, 1124888954, 3015715321, 3554830011,
        153164314, 2571405898, 3088317836, 3826710038, 532463221, 2174408986, 4066384743,
        2858347925, 3362316763, 3912725306, 1672655485, 747559434, 2494848220, 3353179599,
        2958541661, 2754014801, 2253228000, 3548360599, 2532574632, 3609949183, 4224112455,
        2830762232, 1638592699, 748357099, 2027377618, 2154359009, 2042715188, 2328113060,
        2228778844, 3805284055, 3740811424, 437279916, 2305090412, 2502181871, 3285232891,
        3972490704, 3821166397, 3272678301, 2818983671, 4257635933, 1730183078, 4193248424,
        1863033893, 2751966968, 1985590742, 1553448103, 2731396486, 102894954, 1596356734,
        2399109494, 326183031, 3303826610,
    ];
    let ys = vec![
        1675796150, 1752707855, 2960577702, 4246206199, 1769535683, 1968809225, 2828046910,
        2881173858, 4049894594, 690462953, 288094502, 2301238042, 171278398, 2941234911,
        3855716963, 3569445656, 3999649666, 1033046275, 1441788099, 1121368236, 3979411258,
        1744237927, 2218358768, 3293576320, 3290293896, 2918243870, 1271587143, 1530970846,
        1057501000, 1208621673, 1776318661, 2630121830, 1577699073, 3947123592, 1916313897,
        3189157970, 1684300643, 5245214, 2973935012, 1013692937, 2575458340, 1202811269,
        2350985644, 938605227, 710807110, 3840777315, 2476378686, 1408221563, 3963538750,
        1495981337, 345677390, 2267206171, 597425252, 3652332994, 1484311898, 395641995, 508511757,
        1756437663, 1140313927, 4146891666, 1764315654, 3179667093, 2753886170, 2955381796,
        1486042517, 194560773, 4113616196, 3870970045, 687965138, 970031260, 4029682995, 652798493,
        3718790353, 2790548419, 1973920939, 1737499520, 3093968446, 4016940528, 1440510403,
        2896783742, 3442955437, 3111677005, 4265014223, 2141411993, 177598581, 1546615872,
        1296900550,
    ];
    let out_len = xs.len() + ys.len();
    // - xs_len * 10 < 33 * (ys_len >> 1)
    test(xs, ys, vec![10; out_len]);
    let xs = vec![
        2699110155, 1597521691, 470373633, 1547603733, 1114505968, 121868046, 1203637014,
        1508031395, 2678363006, 1428373366, 181016145, 2228522822, 3784155833, 1174663302,
        3119880811, 3351843127, 1893166310, 2733160757, 573074872, 1444139090, 3771161592,
        3202218806, 1184188558, 1337716051, 2651973158, 1523269291, 3416369561, 374511279,
        2679410392, 1510022750, 228616166, 4003251265, 4290642350, 3280834410, 1463007103,
        2311946289, 160203186, 1585276951, 3812024477, 3976220702, 3453132955, 903478724,
        1692984648, 32969770, 393253462, 2089515635, 2580037721, 1368262724, 3975524017,
        1095890302, 3362835893, 1467244702, 3126524190, 1558041706, 1473844963, 2931771668,
        769941843, 1383766743, 2048229827, 3587516656, 744923988, 3114188668, 2900631137,
        1550641047, 3971430916, 1024708451, 266103704, 1961354549, 2996989736, 96509114,
        3209890269, 558760343, 1942895993, 3030238742, 3901981217, 1553802266, 1100766439,
        3617908428, 2903765815, 160559154, 3223711195, 1505354960, 3400362702, 1532921847,
        2633984159, 2547091597, 3753857128, 1603256426, 1467979288, 834683287, 883770936,
        2091938738, 717946381, 1738927478, 4212395432, 3554713903, 2891799196, 2460462345,
        1068611661, 1983982847, 4254702408, 2862607717, 205351503, 899537845, 4178691861,
        2027719370, 1613590765, 1667586567, 658709687, 569869145, 2542265621, 4018309335,
        3115945617, 1860868443, 2042873761, 2857432666, 3454761191, 644158605, 952236065,
        1246066126, 1054146509, 820815201, 4116210106, 911797864, 980581305, 3662945636,
        2395465042, 2988547838, 1592529958, 4123985797, 1086072833, 1344358819, 2713461665,
        1166149285, 868088866, 120572741, 2719927699, 1609748024, 1381464015, 2371158669,
        2027765235, 2167125167,
    ];
    let ys = vec![
        1088368182, 3374520919, 2135624591, 387360487, 3348241848, 2559227752, 3399060139,
        2714380393, 371475119, 1878664574, 3306012397, 3678253780, 2537332523, 634258529,
        2378309044, 1907416933, 2176550942, 3624058493, 608851538, 77324946, 854257549, 2563267740,
        1842976277, 2560652658, 1177372492, 4164431297, 2857340159, 2813781292, 3608170666,
        289363804, 1276568988, 1858470908, 2027103570, 1210716416, 3885179582, 980951621,
        1332461771, 2439102632, 78855299, 1535655076, 820717475, 1372739985, 4277759699,
        1928781862, 2056547589, 2689637269, 3487926306, 1712399855, 2387894324, 1345157890,
        420194957, 2408734980, 1088476282, 1237271902, 1570597541, 1299046081, 2179334980,
        3757788366, 1320170918, 2220338411, 3413493273, 4047658929, 1004605073, 3758106669,
        3623304103, 2595195415, 3392723185, 227342906, 3297612463, 1577658966, 3646845515,
        1442494023, 1805636027, 1293916606, 1856823520, 2157779944, 1701394115, 1586957718,
        2203990942, 3794477956, 470446365, 3294563814, 2801795027, 2712013665, 1473818504,
        2726878536, 4276109446,
    ];
    let out_len = xs.len() + ys.len();
    // - Limb::WIDTH <= 10 * 3 || xs_len * (TOOM_8H_LIMIT_DENOMINATOR / 5) <
    //   (TOOM_8H_LIMIT_NUMERATOR / 3) * ys_len
    test(xs, ys, vec![10; out_len]);
    let xs = vec![
        561532292, 1489901668, 253691236, 2318497628, 4251899866, 2953100123, 2461942387,
        3249119706, 369296206, 4217598289, 2953582842, 2377320700, 2568035293, 3298340549,
        2920237456, 546954422, 3577037488, 92033404, 145112422, 2502470868, 1400281201, 2303329463,
        633903343, 3944799609, 57410139, 3300617501, 2988597979, 3756577241, 1111328153,
        2315706065, 2359556880, 170569603, 1875977300, 2265470483, 1673672630, 2694260146,
        620660163, 4086502272, 2268845329, 2531408738, 745892765, 2985301421, 641961881, 620799476,
        1513471210, 2206613713, 895576219, 3432428917, 1326478424, 721293050, 4129832181,
        2328492091, 790053303, 1886834609, 2560250292, 14318242, 2263105643, 3768652300,
        3685567034, 1053183071, 4035043131, 1140590999, 1312717632, 820131789, 2381319255,
        515196511, 2436315339, 513976227, 688721295, 2969875582, 2843970288, 567346371, 2277297382,
        3266747935, 3125131739, 391700432, 2628083321, 779071641, 2971551059, 3314957816,
        871191953, 3336232721, 2709555815, 918246312, 923872244, 2827827195, 2966239254,
        1586350108, 1024706608, 3525365202, 594940169, 1872199600, 3239665333, 694926057,
        4271587637, 3916707341, 2190558956, 2300957253, 772629754, 238192213, 4247448230,
        3565892036, 3184365211, 2516885224, 3979985839, 1180780557, 783722885, 1061155274,
        3798456603, 3320505371, 589311966, 1623819314, 1001947009, 4232577387, 474033387,
        3930737007, 1729002759, 3148522805, 658463592, 1424102704, 2305467923, 552214960,
        1642169523, 2066768192, 3794357111, 3557589618, 4204044663, 1778418301, 1181058217,
        1612951946, 588858899, 3836952607, 2977777237, 9660119, 2962495164, 2992962211, 3923151463,
        3345257705, 2981383558, 2363319525, 3608470059, 874691575, 2586822309, 912499640,
        603852379, 1888867173, 2770352234, 4238262229, 3877831016, 2596074823, 3663087235,
        542677879, 228437282, 480155344, 709141324, 782255006, 2839979153, 1271748198, 1031245745,
        3053801112, 3462023195, 172164778, 3874269611, 3279470898, 4076666435, 3596981639,
        810288236,
    ];
    let ys = vec![
        2267307147, 2856749182, 90961593, 1052868712, 3437758783, 899762302, 2825414504,
        3100252964, 214994098, 4262558841, 2740902902, 1743352008, 1922058509, 2975766063,
        3399126202, 897115238, 401142729, 1715015464, 244955103, 3177992227, 405891649, 1768495060,
        3524094602, 4080016656, 1432684874, 3397000143, 434821341, 1754546815, 4094846559,
        4286153335, 2240106918, 2310322076, 1713831329, 1428414845, 2188185809, 2111765503,
        1131727372, 929039425, 465389857, 2677898170, 1160632541, 3376736943, 491317513,
        3242464822, 2045506450, 1242019843, 3965879224, 2484620055, 3447163057, 2809067396,
        2409780789, 548871240, 2024164190, 4133800101, 105887616, 4257692355, 1942633927,
        1532037864, 2395107706, 1815832330, 3470252735, 3388820081, 2275739186, 2499364631,
        2076801086, 3670985009, 395675635, 4219873512, 338672631, 3757753689, 730801911, 529959909,
        393050276, 2506914867, 349901023, 889932113, 2359995672, 2260685091, 3193258383, 993644814,
        660499678, 4213349264, 915065087, 44382277, 1138965336, 3728412916,
    ];
    let out_len = xs.len() + ys.len();
    // - xs_len * 6 < 13 * ys_len
    test(xs, ys, vec![10; out_len]);
    let xs = vec![
        2628750713, 2361938253, 4208887130, 2080756349, 672997060, 2130716095, 4212789307,
        1848408220, 3496438901, 84923093, 3765911616, 1894564551, 1611354899, 273564832,
        4150644671, 3064400972, 1543250045, 2858928926, 1070491873, 1579001797, 1184344436,
        2022081087, 579467674, 3617124184, 243126922, 3969739693, 3428743965, 4195070089,
        3234082950, 333482038, 2496442330, 894715026, 434494401, 2735937738, 194433417, 3547773069,
        1310458322, 1092526211, 460831665, 314882384, 352225614, 2524634920, 3907974253,
        3587596708, 90585625, 3922151265, 2706453821, 2479984430, 1899379393, 521798300,
        3544490367, 4025847744, 520557399, 1960228079, 2440638916, 3922652110, 2874111917,
        3780219346, 1155970954, 3101729918, 1154605497, 1641746684, 3885558155, 713658859,
        2298415211, 1104859444, 397648670, 938276629, 2245930839, 351999985, 3962599907, 162580649,
        4135267160, 3893533927, 708603373, 3649893874, 1549341047, 446919848, 3848748260,
        1193215655, 1667453481, 4263900238, 3083741929, 569862864, 111540402, 371222591, 836814821,
        2523967214, 3373518119, 288800478, 2983910658, 3822451776, 3717238299, 4103554210,
        497321656, 1267537380, 2210886058, 393666292, 2341926460, 2993069655, 3449632275,
        345728673, 1850135319, 1546568315, 349065480, 4148532822, 2743969263, 1135023914,
        856540508, 710683508, 621037301, 2245404525, 1375763902, 4230256152, 1103848377,
        4068950000, 2774111626, 4005998377, 1420452414, 142442998, 296389949, 1793483671,
        3236856344, 1470778143, 2199111141, 1485252921, 3021831668, 3409728715, 494048497,
        425352623, 547187992, 307378564, 1878128309, 3632431108, 3608263098, 3158948042, 268203532,
        1889965332, 2413564070, 494017444, 4018318246, 2256416411, 2325799856, 424840978,
        1475143253, 2578705133, 3454058108, 875893914, 3369487214, 2161583703, 2368049199,
        3710749831, 2234731371, 2548143256, 1212646047, 775618131, 821458424, 3027168315,
        841398247, 3991240853, 2094376383, 3587145176, 1943420573, 781156526, 2434343084,
        2126213029, 2402207510, 4019808646, 316909832, 2750686513, 2438176721, 308346316,
        242903105, 3531437189, 4095795963, 2087963376, 3007755141, 1683404210, 3086330285,
        1333246101, 1581088323, 1356633529, 3666603849, 540703941, 1410918479, 2987931996,
        2750320701, 3483743338, 2503688388, 3308034421, 3019960566, 2668657879, 2363438262,
        1470517413,
    ];
    let ys = vec![
        2312659839, 2350424241, 1787407270, 1271425122, 4187967770, 818645453, 3539315256,
        2178962268, 2575529612, 3589703821, 2051328589, 1350506812, 1181962471, 440960359,
        1364212437, 3414960630, 901255513, 1225743051, 2301315145, 1970642256, 2850715818,
        3128888797, 2317420929, 2155667782, 1962983120, 2710186451, 648444928, 2272821232,
        133989660, 3141011857, 1529770260, 802759102, 2173416392, 1305065341, 45650077, 1082105231,
        1602486318, 3755990436, 1936896216, 2400713018, 1591016508, 4068454220, 3596573883,
        2619324298, 33580971, 2286577695, 3083324417, 1169438566, 3225233768, 808739442,
        2766243970, 3455083573, 1549857550, 3592398125, 2248831497, 3521856807, 1967034,
        3078700295, 1346379862, 3820864333, 2903766704, 3884607466, 4174763992, 270916374,
        3218398044, 3434381035, 159751999, 2768080251, 2464394277, 566049661, 442155673,
        4112913396, 1456961327, 38309439, 1525792638, 2372197825, 1956558568, 4294769490,
        3096019721, 2031664251, 3017984223, 1381760341, 4260655051, 2253457354, 2984264086,
        1088854315,
    ];
    let out_len = xs.len() + ys.len();
    // - Limb::WIDTH <= 11 * 3 || xs_len * 4 < 9 * ys_len
    test(xs, ys, vec![10; out_len]);
}

#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_limbs_mul_greater_to_out_toom_8h() {
    let test = |xs: Vec<Limb>, ys: Vec<Limb>, out_before: Vec<Limb>| {
        let mut out = out_before.to_vec();
        limbs_mul_greater_to_out_basecase(&mut out, &xs, &ys);
        let out_after = out;
        let mut out = out_before.to_vec();
        let mut scratch = vec![0; limbs_mul_greater_to_out_toom_8h_scratch_len(xs.len(), ys.len())];
        limbs_mul_greater_to_out_toom_8h(&mut out, &xs, &ys, &mut scratch);
        assert_eq!(out, out_after);
    };
    test(series(2, 86), series(3, 86), vec![10; 172]);
    let xs = vec![
        4161517722334671206,
        271035878974614969,
        8768264966582261304,
        8206956804546718361,
        10016740860128464264,
        2943682457917422384,
        10577659840261915262,
        12098681961003341371,
        2525073961085508373,
        6868684266500244649,
        509821878609210517,
        4263249474085213536,
        2307565444887817803,
        12419028787241261317,
        1281995004584322618,
        13869993964458308221,
        4485392892470363180,
        3274613913599014818,
        13075432300049036016,
        14042578030952079199,
        13098932791585915075,
        10142506622182970580,
        7251838551095799764,
        17051632328075633635,
        14834683551906590335,
        18022997779550454484,
        13851155116066438974,
        3279920275984726839,
        12575373964173554443,
        15489604937489489906,
        12630529117895897413,
        9562379919499143009,
        1417878505992996127,
        2188363987094684136,
        4744951957683006311,
        12112952790370550632,
        313413052918057660,
        952838993607855174,
        5933080761807357068,
        5875775551766205334,
        10228588026136726855,
        13111641204516926240,
        10636665232562365918,
        11359964631071199362,
        5929704785320756798,
        7890881054270407934,
        4884891330151666074,
        11055829837821054078,
        13707765469312479203,
        8153558212434726394,
        17445193585880639275,
        6568289716541023323,
        8041757936108402209,
        11089742802624534358,
        9104866424438942973,
        3236275382520001400,
        9213626463300221545,
        5359296447813232573,
        2888775200925828643,
        1504166968227419931,
        14327007717613163305,
        11802896026004225094,
        12726419078417922871,
        13309155468447837337,
        8586421913645886721,
        53962250520164792,
        10299535356260218467,
        16946113957982976032,
        2902460381404773190,
        14757465720632393328,
        4285719983639600380,
        8437230965528545912,
        5716398831975234496,
        1373020012523386515,
        3326027605041066746,
        17656221602314109866,
        5927567778944922379,
        7395768072445629410,
        11551011221061348004,
        13862329630891761456,
        3443745263810155735,
        497965567194021216,
        13073929868627981515,
        9340721263069758697,
        16189911797862953019,
        17331477506134450185,
        18441976800868209749,
        3733349995001197864,
        6937510789920909911,
        10459182483341515090,
        16282716012969111817,
        3142838808933013004,
        176169927348158611,
        11447076894000834768,
    ];
    let ys = vec![
        3898028307372664956,
        17056541935478225194,
        14004255653437064260,
        5500365157672511509,
        15774417221201329293,
        3229812365626959565,
        1542674716041014040,
        7356251598468809943,
        18181760582149085284,
        6447899299954117957,
        15228766707939040914,
        15272444333081468110,
        8256864946368840840,
        15131537266446006793,
        15615697223616434527,
        18149135087211146951,
        6359898540214993921,
        11306735121000975748,
        10447887135010383963,
        12772438236294882417,
        17631737056955710770,
        8945404460793598129,
        8945720889114856152,
        3648711115155303988,
        4353348842999127960,
        2258094147328762698,
        17154005505580115535,
        13882701371593165208,
        1610163839528654069,
        15350954595089578211,
        2071555476679360064,
        7797386300145290156,
        12827100752536039252,
        9294676638100895403,
        13194197740670114341,
        9490868657650122292,
        13133123495028388830,
        12350221742051084451,
        12424378851382358824,
        9807292823459903392,
        10987641767148832341,
        10914994897211362878,
        828242546480310184,
        18006801931269403354,
        3042908768715701160,
        8117699035539485321,
        11944855102415629844,
        7384949013429384602,
        11066738683960763872,
        14686958392900209441,
        16412025437157422416,
        1334344044228684681,
        1631366399820348565,
        18062594111889109095,
        5175299421808157128,
        16616812968596909641,
        797326939277169478,
        14593183003025528412,
        3580961852669434633,
        2104948106588459323,
        14322976299272137248,
        3536903766355663369,
        6932211742640251008,
        17616766237027326857,
        1477865108082927148,
        7817082715310166375,
        16183969129154492111,
        18146981620947356859,
        11618268397687338183,
        15294321769160092821,
        2447614867702883346,
        15261926111061449320,
        4029723450982123355,
        7820711996327940306,
        6188156586792352365,
        15703528769184364862,
        6698415575574578533,
        7770946582061166480,
        3543987370105940918,
        8845414905041844753,
        13110356713999163167,
        12862812457872444435,
        10749027774576978236,
        17822296942008093229,
        13898152040175560707,
        1879212271519144526,
        5428215269251527991,
    ];
    let out_len = xs.len() + ys.len();
    test(xs, ys, vec![10; out_len]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_8h_fail_1() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_8h_scratch_len(1, 1)];
    let mut out = vec![10; 4];
    limbs_mul_greater_to_out_toom_8h(&mut out, &[6], &[1], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_8h_fail_2() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_8h_scratch_len(85, 86)];
    let mut out = vec![10; 171];
    let xs = series(3, 85);
    let ys = series(3, 86);
    limbs_mul_greater_to_out_toom_8h(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_8h_fail_3() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_8h_scratch_len(86, 85)];
    let mut out = vec![10; 171];
    let xs = series(3, 86);
    let ys = series(3, 85);
    limbs_mul_greater_to_out_toom_8h(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_8h_fail_4() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_8h_scratch_len(85, 85)];
    let mut out = vec![10; 170];
    let xs = series(3, 85);
    let ys = series(3, 85);
    limbs_mul_greater_to_out_toom_8h(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_8h_fail_5() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_8h_scratch_len(86, 86)];
    let mut out = vec![10; 171];
    let xs = series(3, 86);
    let ys = series(3, 86);
    limbs_mul_greater_to_out_toom_8h(&mut out, &xs, &ys, &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_greater_to_out_toom_8h_fail_6() {
    let mut scratch = vec![0; limbs_mul_greater_to_out_toom_8h_scratch_len(86, 0)];
    let mut out = vec![10; 86];
    let xs = series(3, 42);
    limbs_mul_greater_to_out_toom_8h(&mut out, &xs, &[], &mut scratch);
}

#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_limbs_mul_mod_base_pow_n_minus_1() {
    let test = |out_before: Vec<Limb>,
                rn: usize,
                xs: Vec<Limb>,
                ys: Vec<Limb>,
                scratch_before: Vec<Limb>,
                out_after: Vec<Limb>| {
        let mut out = out_before;
        let mut scratch = scratch_before;
        limbs_mul_mod_base_pow_n_minus_1(&mut out, rn, &xs, &ys, &mut scratch);
        assert_eq!(out, out_after);
    };
    let out = vec![10; 905];
    let xs = vec![
        6809535447687914471,
        1103330005972507068,
        372788324988356904,
        3310988417383334329,
        2768136713654061049,
        17867128683005223868,
        2238946582723710409,
        3081056558438604150,
        17026613024401611127,
        1744473747933758430,
        4884032754714318833,
        5071706901657208751,
        12689572021505695693,
        10978480551041753799,
        1800129976413451867,
        16286820002622524539,
        7281000574726170357,
        4219678291438654832,
        15700081839772688751,
        15205907878248907247,
        14393184334614991066,
        14756848254875178077,
        4793637008204029812,
        9858797803427744303,
        16884118866110086987,
        2330774877124323542,
        16233995226797389662,
        6091870787973288330,
        5905123672617126566,
        6532637120046850310,
        3781012952181700512,
        1147955258215944149,
        16381863291477272057,
        13470194198585905847,
        2807745192926696062,
        10417338102018291024,
        1182003379390951531,
        10572015994470653864,
        11256589899556451377,
        7047560236256902575,
        16809106838958682299,
        8167397083217775737,
        10405120914265859441,
        6050515699222625782,
        17283763639396397450,
        12481792043865481917,
        10515115098175408116,
        3547231139273206009,
        2955870806881464447,
        81763485884574417,
        18038197306999979657,
        2697663113419170508,
        13640328591104580425,
        16380387086795482436,
        5937997179794219672,
        11280227770841311656,
        15247718447550407960,
        3356207189433327172,
        9295483089881333487,
        1048090603086884014,
        17630347864859687326,
        16292502783826501772,
        2554058376409680235,
        8746244974811121511,
        13601151617020904249,
        11424464616482878222,
        16592787416774613845,
        9541965809588758702,
        10304263209519954371,
        9718232210246640079,
        3964199137038797179,
        1133689222449100290,
        11762544598027160268,
        8892566166642784135,
        4992855273094478682,
        3467095579457639126,
        17527051578764837605,
        12788831523184320706,
        15884832626408420422,
        15445614371886990386,
        4605408054811466579,
        9468992277252416191,
        16059556135741327423,
        13703354904914355468,
        12651524370372775470,
        12076530382821868093,
        9203720925482564499,
        6751521177612318975,
        14730048400600245248,
        9583087821356332745,
        7635737274166465491,
        15909961927185268608,
        15152412967792641434,
        1117148048506231546,
        15036765086001151283,
        9461075977858154319,
        283984775170615496,
        10870934978120381565,
        16402595751643852267,
        13300950714409955136,
        16269834104766854165,
        3823800040156766433,
        8001239710257605750,
        8854456135465589491,
        3559503577426585699,
        82907824154359285,
        10429508186090534661,
        1795271336507026791,
        16052612691484506490,
        4449527904102534257,
        6652828614119627650,
        9436314767260323923,
        14084834581845197080,
        3924506166878684856,
        4006253714000661354,
        16835787368527074469,
        2939573978616449941,
        827685924983906914,
        513438846841933201,
        5499086145667050935,
        5970432596053952196,
        2357808915999726215,
        13130152994826263557,
        1029371010558121271,
        9673453535289645591,
        13630045027309361503,
        15140438118448998733,
        14475237670869667854,
        646607524428232975,
        8603154126726854477,
        562632002079264944,
        11132087168116199036,
        712016548873706286,
        1162552953778143562,
        2720821187699295729,
        12629405027198622023,
        10085431077125120126,
        11036408623217209328,
        9412034889605831573,
        3580947814659655797,
        2384617485952276249,
        11991332791506170431,
        355732385395682650,
        11957596579072080069,
        7637503009753705463,
        8641065560841157989,
        5385185370436588141,
        14613602605895034796,
        16149929536213837897,
        14926494841744326505,
        8478550502351441150,
        4754411934055301811,
        8446559217491804253,
        175563428577047295,
        13317411913787179089,
        1999388610388624930,
        11997574948104529333,
        7709394800320180604,
        141106960118475071,
        9185076664574888313,
        14711830716786764563,
        8646328906792567341,
        6435770954980752754,
        10889857927018380675,
        4856598903599320278,
        5559749328765755744,
        55448005653193385,
        18376290341644060781,
        5411230779476997204,
        3208977317832509473,
        17939099296520163690,
        14610289283912272679,
        15727344360076838968,
        17691225907664733828,
        6277158099136871988,
        6227268882497066304,
        10537912046761025798,
        4569425383713839933,
        1540443185169512503,
        8083699057632389568,
        7713824171800942769,
        8199521127345296820,
        18024850838783811685,
        11847360547777377558,
        4462968389243655548,
        3995475265600836322,
        1735988164410255922,
        5169863977080869823,
        2205341702567171145,
        2235183401932173280,
        4237083636488253034,
        8651273126396740151,
        7677914001769310460,
        2488550701988271373,
        6362894001274871299,
        6938141915629282617,
        15289077957178314583,
        11369273085874533146,
        15146552950762343609,
        2250130405529921684,
        4507408042222997556,
        12067657188384750699,
        753594982979265248,
        9819871477024072995,
        8074675523776512539,
        17863686634271613087,
        10148559641897798765,
        12078707926245944334,
        16056144107847929568,
        11477562644132166451,
        633839849054735197,
        1974937519863601198,
        9431759751115500349,
        16231263638095445675,
        2852944099010004296,
        10805052896366212239,
        1090853228632071464,
        5943661063504137996,
        12279437176065016726,
        6997180572817064008,
        5167976820090909749,
        7252605595265132660,
        2377574067337995256,
        898853472418766215,
        15283250736100608687,
        11681971203683297764,
        15733049786589504720,
        1874550821442898161,
        10775751683446032879,
        14346795044143203654,
        7996691180178068337,
        7988670994586391460,
        15438536777951336029,
        15583275281862893881,
        1645086929138484726,
        345744887729039216,
        3687893952132688571,
        10659362456954960532,
        9602393538964870338,
        16520689555988157334,
        12546754178798486611,
        8040849778898919845,
        18425567932309437192,
        7524804173796997825,
        7482803818757957426,
        2159381166607938760,
        11953191500503254263,
        11662381083106843270,
        15954507061982303817,
        2311355641538841753,
        6262214959978467810,
        3205610786437952171,
        5833762588841339039,
        9584891950834271718,
        16431258786750754174,
        11295777256302945260,
        10262743010189168916,
        6579167273633548585,
        10655344951630924167,
        12426159972843518796,
        13809556914733676017,
        10421408021758282646,
        13742014587503715343,
        1213383935103977733,
        6155186430649126184,
        180686018379637244,
        6671736391598895649,
        6711423272868884639,
        10567222126096946863,
        236255051969434496,
        10538599139522078386,
        1840416208489527534,
        14920772374579070414,
        12181583640158673672,
        13150385378549419551,
        2385161800547570055,
        7427796248918503499,
        1086611422915217495,
        1034188908993802486,
        17895224758095668351,
        5605583861310416260,
        8276090981819119594,
        16774347628768167103,
        4438689443393994418,
        1445832965009646259,
        12020209207509332983,
        12101154623632027374,
        15952145139262310731,
        4569147745515423074,
        1032438357998575006,
        7719204784227574899,
        7620365528941515938,
        9134138337791325501,
        17595820721307989341,
        10837897611871188641,
        3628300610284347069,
        6056782082517998847,
        11889121953261384595,
        15658848261681587671,
        760281128415892566,
        9901843341271410215,
        17997287540098693599,
        11471174400256577350,
        13773435664609284401,
        224645455536727129,
        3248370257584471677,
        3402146064818516495,
        15419892046652096835,
        6817408051904455983,
        13462567452174648864,
        3865219578658340818,
        6401692728490725482,
        12241061753482985847,
        18093898726091231688,
        2085695270753393724,
        1486152210073874278,
        10962392580868374592,
        18233050379629714175,
        15440497426454102854,
        17029248778709090701,
        11943840019902256468,
        1838494147415724342,
        10624384750477063994,
        2802400104447544796,
        14482700272009891532,
        7488752288284248623,
        16689053330367821624,
        8122258539396763672,
        9378594940099937779,
        1953829770254669263,
        9095248272317331200,
        8866691394084862202,
        13201017990930506125,
        5874101266603325988,
        12758355031414108476,
        10906401543018450367,
        7070115242582431545,
        8015463853343573521,
        6337419761897564053,
        8245569799737529081,
        2143044650526073628,
        10001469296387640338,
        11129612211343148678,
        9846592166231801208,
        3293328206977069772,
        8271586651454757561,
        2516303987084978983,
        18226852422794010543,
        15105013013529789613,
        18154225228636318154,
        2013990863661732083,
        1784747160358384741,
        12567581004427688912,
        12324983884662973603,
        4935897487517618842,
        14994876132441559883,
        3460392282827044973,
        178151701108469307,
        14330685158880278679,
        17005759398710652579,
        12918079073431941444,
        14737890013936547836,
        13116140709796209375,
        4406659418908136238,
        12427086661031458855,
        7492548499951595477,
        16111633531233494957,
        6547469955282364389,
        10562100669850668222,
        15418878616411295006,
        4956757255402949102,
        8013302230254643702,
        528640010877453099,
        14207088120532813066,
        16751882716911936366,
        7949419147113034512,
        1638837693666608472,
        13878268226712426549,
        8273407385523164786,
        11509174550235804944,
        15966833959669295468,
        16501367189938100697,
        18237618137386130737,
        12449896376279048977,
        18110666911456042806,
        3193062725115205782,
        6188346541223558421,
        8058450572610971381,
        6102995183728564982,
        1217219394955358461,
        4696899915064498873,
        3897448246350199671,
        12662172956137287871,
        6610124623177066401,
        17355303367288349560,
        2046855789488294262,
        3211014904560014361,
        17115241076245460723,
        15298358927000333664,
        9182539314691893135,
        5164209594070862585,
        9062044958688820752,
        11123512834458221854,
        2659061244791686400,
        6806035717656270289,
        1027290028815967628,
        9405947735639936936,
        2540349601344589130,
        6895259978837037548,
        8904474026325226434,
        12273615493885622366,
        8677764880724506780,
        6002959367751808149,
        13123965742105252412,
        11735284964695785299,
        2417001610052848193,
        16769279424767143997,
        7098399088498274096,
        9910748236374366737,
        11111297830299095939,
        13640041663283902885,
        15915174780574330612,
        10929481089808875146,
        4581632826936763294,
        8581099749824253588,
        12645736569302882821,
        7359153121066395683,
        114042912187761457,
        8021867264848320576,
        15492861144941287563,
        17574065125533105845,
        16374369432221228355,
        10459169993431587286,
        16366034776295832243,
        2627084718398353897,
        18099240392705942111,
        4326916362018195403,
        6552915731272193737,
        4273219172539573763,
        12682979794809012349,
        2630341686377453497,
        2883112777041844122,
        17280553753834600520,
        16523493292363775458,
        4645653082909935710,
        10857150457399097857,
        16240900012104288727,
        13936061320958754124,
        7489056125188749067,
        3240618672663587347,
        4473897013556224538,
        15959787311013455542,
        6807910945586226902,
        9410336371061067189,
        6906088553149840730,
        5605158148442495482,
        884531864210940213,
        1872297304786275835,
        313522309067627421,
        17334018801756959340,
        1675436862731542012,
        8842978529412038193,
        16491099153488944340,
        3255602365380029669,
        4091823593236384660,
        9059086866751700824,
        18074592049968473162,
        8939611511088706324,
        2901954556715326677,
        3168063820638083277,
        9642457237234984168,
        12973730047202029231,
        12383420012085361069,
        11980461635855055601,
        815939574328079776,
        14011981531029282088,
        3819777392902083260,
        3664266433302366398,
        3842827072078938508,
        11680613555445372946,
    ];
    let ys = vec![
        15245088662193948010,
        854969528224537163,
        192457876290468361,
        3156774054099849881,
        10102117358735393641,
        13923135497401538045,
        15603007686998930972,
        3707765480829539463,
        1075990372015045994,
        4440028045035707188,
        779932550205535682,
        13284596850012603887,
        13447370325749987403,
        10657005451799608034,
        17344058779081327933,
        1801131630646010099,
        17879455113972297046,
        1049662270419803525,
        17887003202529550415,
        13730724178286439296,
        3086493866184691051,
        7455503161286080904,
        14945249663072669446,
        7413071270018261565,
        8165098975144402988,
        15667870805615006559,
        4534237642686726425,
        5675059133984408369,
        13542693529471369730,
        4650690134857994243,
        10593876026982724440,
        8719234160809710444,
        7340192483727047710,
        2225660849988538666,
        3260628781823840386,
        14784063213821786553,
        13478324037708856111,
        6239844587086244103,
        14508626048519473050,
        11443816492520902359,
        7084448144752764341,
        11673478635762496725,
        13444020463604694513,
        1798574113181758005,
        15195278749704748030,
        3490272214933312037,
        15632500462832370824,
        9808665338648603851,
        6377980234800091876,
        11306384233660763805,
        6392788317448223882,
        8005181869701567455,
        4601526777105113530,
        9348184476999479133,
        16105441815997897842,
        15373735633778437011,
        11733794529384137433,
        769246272107807645,
        2922899274256775805,
        16218486247871807873,
        10650657974127272786,
        579665301817927565,
        6403006378940431337,
        10150254532952843560,
        3736822004545760197,
        10244207440138560761,
        16631379436671010056,
        17418302422321190629,
        4844439457855539440,
        9662799133272397874,
        11622100630061039998,
        11017257064923257696,
        14025546287952884200,
        1170766120552674008,
        4852413824670160293,
        18019298735978800767,
        14042374992041286164,
        6103187929964524269,
        5988592592688695870,
        5579172720281387479,
        10738878044274955012,
        8401646271610146442,
        12016061916593958227,
        14752402557741497038,
        5053283107906893264,
        12910662726197463795,
        787526459034857809,
        10304827788120361107,
        8387521101013404665,
        6030209567663971422,
        7511028869236306454,
        11105170944119024313,
        2911699195421772292,
        11710398806568443147,
        7599646386487625804,
        2146501359265516686,
        1193294087739295886,
        16419769173966961854,
        14779980297792837632,
        6286361066120350249,
        8246126699673376536,
        2339493649448723726,
        12383521129608538925,
        17459816050942292574,
        7213741082075285427,
        14702683527305456088,
        17849030573001874153,
        3273901152373442943,
        10086273715179643444,
        14351251935054659627,
        3067622597087477151,
        4241957707372911307,
        16686513037697490920,
        1503886102490162470,
        4222986769290077389,
        17209928444872897872,
        10064374817012298812,
        1391022681726221923,
        3482099619102309134,
        151151415131464647,
        5477310851692317777,
        8185741896741403527,
        12297179519749775078,
        6980896315258250234,
        5491311995173541969,
        10908311176531272611,
        15140263006374103771,
        16292302828281485620,
        13488663273854028028,
        17078235461511918753,
        523009743565281503,
        11105648925812514991,
        13827146014280242829,
    ];
    let scratch = vec![0; 964];
    let out_after = vec![
        14914577666128062141,
        12068273989972843735,
        6116694005478833271,
        3562611869773989286,
        15670691724611128823,
        7249128461000381996,
        15151435496873338180,
        13770931346629219578,
        1187535282027344550,
        16236693325430049515,
        2991034239214163143,
        6018061923835566187,
        17284480928832658068,
        14451687627520119240,
        8458802009939241800,
        3578663573390196265,
        11053350197861111769,
        10545908371462300770,
        12247772832897108412,
        4468839684066577960,
        4549770468763696098,
        17752125097317086921,
        4544887864950226436,
        2945143834276229802,
        13648049166616052237,
        12600592346334587273,
        166409088005131745,
        13013620684136565587,
        676730163848086040,
        12404388221407599051,
        10705640677758869552,
        4689698885735113463,
        9700890479828306187,
        9036861144548712951,
        8094206411676899952,
        17821082304555823758,
        5054804670485674122,
        16421357706067082536,
        9704734314786980295,
        13265421291054572321,
        4619919032177510478,
        9576547362592346883,
        2025158308184190088,
        4984956647828393866,
        5567014476440543897,
        7390123682276500529,
        18303245037317000170,
        4525541943295462130,
        2509303616856320788,
        15659980472438820675,
        13570563127509629314,
        13564844957739564512,
        5956170799543933857,
        14572147749185034353,
        15936664264029850696,
        3337746889684993302,
        5339311381497232535,
        791608983511277044,
        8425434054207053106,
        4098815735774818440,
        13011885841922962026,
        10478691500722275800,
        288311354477678992,
        16688724556777848799,
        3854559944896493088,
        4787579643539441431,
        8914365544703418944,
        14867687936657046038,
        17948945926508777996,
        17265394933400453261,
        6520518082305967564,
        6823369778027186436,
        14260127132198472109,
        5636179564496979683,
        7510494946039669953,
        7445089968619727375,
        13151208295034342548,
        15327531597402220486,
        6165165423748958807,
        7649301241163383867,
        932300362699473366,
        10332384620840363496,
        1645779973200198914,
        6819633096753052339,
        8430257326959342191,
        3365328330624946043,
        17882376594601626429,
        7931772344347882099,
        12207106533717173393,
        9553942214328281184,
        1211963965695483236,
        5986284857424925918,
        1725008760284435757,
        13885703170228952143,
        679979762314983844,
        2803926223916590493,
        15129803423596480560,
        5700355505192464858,
        6737697386531803219,
        11676336532156262967,
        11219388591096271471,
        6036126874875117221,
        12759399619486488762,
        1075627928411788278,
        8986609583347352588,
        315614912525710770,
        2968976622267634808,
        9398679997246989312,
        7442449431966307944,
        11074315772256995625,
        13059753170473066158,
        15516530300302448410,
        6694913298086290552,
        12225172653086039386,
        14693751743728804585,
        8946708930559907274,
        14677397241988234315,
        3209366617332728026,
        6979717107269587401,
        4941778982576664589,
        12304520439559413990,
        16634028143562183371,
        8142239299779908752,
        13355303957199352688,
        16647311136213462709,
        4819997788796101212,
        2740900572266512222,
        4917952099793992792,
        15276502006380998408,
        11163544344324054972,
        4805821501449058572,
        8031120161948062386,
        9296985703342487576,
        16564164538495558074,
        4151584010060908718,
        736580097771121849,
        17811365526093310331,
        3584057130812467869,
        15343174508489570281,
        6663904409520260794,
        5148198311285516597,
        5459037875257818034,
        1732839752669203018,
        5817171919148564506,
        14532823584962157630,
        17664910567425460977,
        1606328717145502899,
        7258676117087673712,
        13564853056256859610,
        15566073818913471128,
        17505691249649149638,
        8720681711808526345,
        9299675336460004741,
        12175130218583331200,
        17275662125027429780,
        5254636675017986768,
        4584180635141974747,
        17229718845518394206,
        9903358674827047499,
        14227884446253370289,
        10238808006451695682,
        15776343954783670697,
        3568203177480129616,
        10791516058691563224,
        14667733988945443113,
        4760849120471199172,
        14824266274906749858,
        4738766195452531583,
        18201869520178488918,
        7991042165868231006,
        14001045459551827213,
        9728825906169570644,
        16356820821501653437,
        4624890364800751139,
        3202345381085270173,
        10825323351814264733,
        12844260821347730875,
        17890471175274365223,
        15983756164008599310,
        9229389060056964014,
        7980425933216172325,
        14689445497065509264,
        3947266856163410121,
        13986662827479113948,
        14086840354569963505,
        13182084671726443227,
        9173479010912744389,
        4592278812024543534,
        15484536985258289385,
        12413608308331869638,
        2694433334520487941,
        1628652491883279350,
        3020979365812132524,
        4761921028454215909,
        9912693764834406519,
        14831540747828363756,
        8079998785196191830,
        11252016841358535301,
        1583433672276669340,
        3452712503195064151,
        3057285240369947887,
        2595743348455770874,
        13596185098573058903,
        11667772373252099988,
        3692370881551772687,
        14912011223264263901,
        3289162294555844501,
        11705734929820375948,
        17609626782993672509,
        18117255127345119328,
        1604093155153663473,
        9302755008283648290,
        4607754412797627670,
        2438148241672671544,
        7767633059920523266,
        3595119232557040143,
        12437965844493362543,
        5945493114625270416,
        1881882682439839256,
        12119104281808262978,
        6340145747352803155,
        13626236273901226360,
        13270069380549785256,
        15850889016963874457,
        340925488865420050,
        14028143098693299269,
        3675481773099673886,
        15132383794361616982,
        12233631174234968448,
        12037785574496977166,
        10737440493925210255,
        17968097042350264930,
        17680205263988402231,
        10859227415540706659,
        1792989334549221826,
        2177480830391393668,
        10338369798118713206,
        11542959090623521328,
        15037173432548249812,
        4664260954029694873,
        9780024773679095230,
        15423051572962932558,
        9219085292170154212,
        3117519792300278178,
        15201618561147315107,
        4493458887555315362,
        7071488025555767928,
        14797673892858036574,
        15761839968701169338,
        944266743877977151,
        7946878991241141272,
        11588285294320616520,
        3085763786364505728,
        382160258300824544,
        5755505265595418236,
        15139252786842523155,
        3984606858609787503,
        7067124810169118107,
        11087518322251373936,
        2145074716108216620,
        13925513670821357445,
        11473752360428444547,
        3033405528387034503,
        7582032677934023131,
        6892449022193017132,
        13657958745036075118,
        6128417560891330511,
        12848284770679740346,
        13130020275156534634,
        15464211076162624262,
        5054410800851766065,
        5483902692780184480,
        4738318744782334997,
        16867167009329238452,
        2678560575986600940,
        2615258434432884977,
        7292976780151019583,
        7128935031167862533,
        18221899987307473439,
        11248303633169678295,
        12738639111676263555,
        3155353877742869694,
        16071495863264781073,
        4264698448823081203,
        7418994727203896067,
        14811917097660163843,
        7385790368475083290,
        664373701990601810,
        13483754102463667701,
        7612123824094233525,
        512453134520186948,
        11691341684613561710,
        13867096232148780503,
        7687434174716142000,
        6803483980923514751,
        13342782916572404854,
        8439201436522607326,
        9757760943763629593,
        5981970754483057908,
        4190757094163155311,
        5660547043969581137,
        14325603855966161415,
        8929633236673768270,
        9297312576387550830,
        1693558263949957284,
        6836793952945831062,
        10347383165146273428,
        14661681337332505439,
        17963531302478114205,
        1709137293854005147,
        3006720702657955228,
        7329219772915910677,
        17104100208631258612,
        1949699737886797762,
        17000436044616227003,
        11324808183485510201,
        13235495906244342819,
        15749585439652205935,
        13303759086698322754,
        14891288338853864137,
        2704605299240507925,
        18013575082547999538,
        2430267745104734440,
        6179542971962498102,
        16185308076341237746,
        16642985193904420358,
        2654686382891502775,
        14166125840594907890,
        13875221482589820174,
        4356811139286595070,
        7880657059321548510,
        9955951238878446164,
        17891804138739212705,
        11829321244704294673,
        11236151306675978093,
        4831011344740993127,
        15572388809599316560,
        17846543352626240390,
        7199845863441340373,
        11121886749027339043,
        4373695674384726918,
        17093036459186744076,
        12232005422016339414,
        17131567018724437993,
        16256199016182051711,
        13475963188288260257,
        911027923034979647,
        6400295517498242996,
        8837389249699670037,
        2811700517251274120,
        513035437159887251,
        15619284911695136524,
        399375572943822801,
        7075743944967114918,
        15672634271222934582,
        7207711819265953469,
        13693046740725554616,
        4027434095315202255,
        3539401330347027933,
        14804329511729011870,
        349324242806502305,
        9296277440807870703,
        7176700326904811702,
        240113067969972951,
        8383831921575151446,
        1722116363229832243,
        11612954111300504655,
        2215733152712733871,
        13493183699196154480,
        16071186765983825953,
        4130784068501749192,
        7338573984508524547,
        10825001941195956075,
        14997934222654203001,
        6505703712429055284,
        7221915757982082460,
        1275601006259924603,
        5672514881351973473,
        14882830470570010681,
        10941637348461954999,
        9488919083136123898,
        14043552860848000866,
        16822079753245337019,
        4714248213100277643,
        969715273010037797,
        15521864292227188985,
        16073105970793932270,
        15708820798300121289,
        13898215817316628619,
        12776496008354836588,
        14999798045466993632,
        3260898735577297939,
        13139436153787118831,
        13332280475731022238,
        17383012290790364040,
        2995641007327542345,
        14136727525433141277,
        17334568127137876546,
        16958828191205021632,
        12417369554667951160,
        3090518852098624195,
        13657230135070531823,
        11743085280276876203,
        3114616886140581544,
        6043287503993377429,
        17550757261658589945,
        17149590190690566703,
        9103040591046283294,
        7946146551153766574,
        6328750617512334061,
        7611645677682650836,
        15633620719663274568,
        9212436143674176261,
        15788907043061834348,
        9498761316246368036,
        7352496413262997914,
        9095543535942948357,
        13109519298804112825,
        17605710606152813899,
        11836475532966293267,
        15936748974921397105,
        1111801049346591026,
        15762144272111321164,
        13004674117006965866,
        11908453630639663926,
        8672383672749866782,
        2088248899380502026,
        7296610571351867175,
        13755695345060796205,
        2999982423435890541,
        4129519642079256449,
        12622156276462967022,
        7111496398765881581,
        14480787311565160150,
        3469939266209117746,
        7438701329152033623,
        11629269482462741856,
        4885777542024810464,
        10705129561986474549,
        12704646697057875,
        6458480273092609348,
        6552269679491575287,
        16764409845541378365,
        5112781772516022587,
        8008989787537536761,
        1103584530948037829,
        16382841334779146359,
        10554805089071823936,
        5435260964051060833,
        9791508728754514420,
        490382068908134805,
        17435419070710567937,
        14195398757764953304,
        2963888499740999291,
        4662041165840469949,
        6744621662455431846,
        13427204268640870135,
        3887747288146601313,
        15443876414310792510,
        14209765039523837918,
        15685162874875886369,
        17718873723062454563,
        3899085415479325935,
        8112448753673506965,
        9870640447409364221,
        11832870733499002257,
        12563697741369320162,
        7101757806260064829,
        8836333095435223130,
        2301074177341928226,
        8276824997793946999,
        2827834280767843126,
        11204234843351418659,
        14057032714422307544,
        16637779355549534512,
        17293043019105638911,
        5235444579393367735,
        7875158171121328348,
        12032932633735348172,
        17577724652120330660,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
        10,
    ];
    test(out, 480, xs, ys, scratch, out_after);

    let out = vec![10; 905];
    let xs = vec![
        12477471680342457016,
        7439834363175998259,
        8364531072478531307,
        7720669117264982572,
        13871089126608066865,
        5538739089658515009,
        13485862549968349516,
        16668965138415286574,
        12815918125584966674,
        14984155091932260860,
        16644766636646585123,
        306590725006293979,
        8429368518159297616,
        9218529725576681069,
        15016741949480768499,
        1670879514738013725,
        8417955162372577946,
        15387720484925756369,
        5110476884223883348,
        1816362467650579730,
        6344760880304946457,
        4582050077501139760,
        11544561500401244826,
        17507375286136941082,
        6780825611959218867,
        2173592718397418409,
        14895320899302250577,
        1732130437372403944,
        2652228013798656770,
        14059887082577558829,
        17320142963333434007,
        14639018477416372959,
        4436670787324500429,
        5494397060264160564,
        14648982379079464056,
        4281784633461453767,
        6798187094949642558,
        6608877920405717218,
        12631712797504769978,
        8551308273612802041,
        17246776198396440310,
        6444397599333607623,
        15925650298696285563,
        10601768410796015631,
        15000906276520941112,
        3338797399168993953,
        1246352375954040582,
        11420394385271506024,
        17068095307090480528,
        7644631914604520606,
        11033781661348318008,
        11117169690184835735,
        258761790873428567,
        17489901188933003933,
        14840505235785535230,
        13227413113638715481,
        16997698053799118525,
        9019797589817151925,
        4493694255366969932,
        9725237995968980495,
        15577413875064628551,
        2280312394933338862,
        225569847977698671,
        2323301829890963353,
        10256874123357921822,
        12539122566148296448,
        9989537237723725906,
        6811992642529374338,
        88727847633051669,
        15695593575052197094,
        2761202085861005143,
        16533039048484256106,
        9351088285086271793,
        4055478109313752276,
        10213434408975318081,
        8762684020269433141,
        5837079994402234584,
        16514518402377369291,
        12378208937452956180,
        14421807887515276362,
        3988627559762405319,
        3918544582422029230,
        4755956678288947720,
        9676877814462397554,
        224961575368542488,
        14611489560386313010,
        5196486534973110781,
        14213777193365038984,
        4602978866967803674,
        11003619085742827120,
        16763677630294953394,
        1684559030448096764,
        17767374982649848838,
        4877949630873126245,
        3682814944580280752,
        2561235719818456295,
        2364934896956055350,
        18263613281445563076,
        4089114060737928308,
        1871768083735193579,
        16678770828678552573,
        4414846026098045432,
        17125831209897882395,
        3912127090800830255,
        14011181803889192282,
        1295043426575781791,
        10420967703804408197,
        6199906131836288010,
        11807153033347357192,
        14265899593105313257,
        8147605267573587369,
        3295330776500509264,
        16732577613226211515,
        3630606949899242603,
        4077531443619740772,
        11812426165247810337,
        780989759563676061,
        16843715221613475518,
        520503200629457264,
        15698734581373560858,
        15717047446773117386,
        11739182472950746776,
        5508507755766772851,
        11277744890514126849,
        7102937323315103744,
        15043426316219001362,
        17264933403548961444,
        4786858792980655802,
        2014728966166520846,
        13049152613696846196,
        8986797982833879405,
        5686978963676857351,
        772150166774986658,
        14517086870053753709,
        14954116996489490375,
        7746391697910526188,
        4606434146793009376,
        3009553593220536962,
        11834331104428006940,
        9959812709547803325,
        9312818550721389795,
        8167722699792448458,
        732874657192970052,
        698773781905867242,
        12119405880051172902,
        17673033350796167980,
        13538870733321508367,
        2664725175527539380,
        7636536994105874784,
        8629987496621113680,
        2000550558774774739,
        13628751714628702436,
        3345724186836801042,
        5296651644730514746,
        15440075910536479523,
        11566524344598274755,
        12142530999548267726,
        3178476168461706193,
        11231993373182544616,
        5535836762655782188,
        1821857691788680579,
        15398067123112832375,
        2959937906512595934,
        9894128090659796945,
        10977676213479413606,
        9896053141077387340,
        4732359585478105829,
        2782537058538126590,
        16748650113511090469,
        16234573623069140583,
        13526364553240274764,
        3472042735026885459,
        1375504011635585658,
        14134901088610758184,
        5812986093773998116,
        18343907802066284210,
        10338195362284836496,
        4538933452393836273,
        7202972851508573831,
        10663048845744919725,
        11111095559628771539,
        15813947556665176307,
        3844063447234634925,
        5652338735091822320,
        15812061039848958137,
        8229753298839583649,
        802923134917039889,
        4829163585440544772,
        15429176731296846429,
        8019558793413477543,
        1452525545607666514,
        15573790577533593753,
        491936016981197942,
        10062863560179613545,
        11061384784011621889,
        1900052214617990619,
        7753731008479284745,
        16019010346473083888,
        10253841258470892164,
        13903408453788781151,
        10169884958783022914,
        1771119784479465694,
        16876761255984683913,
        4507734336556855851,
        1644190668914968903,
        11524855357000701681,
        6768296219427733234,
        10057582861726337550,
        9899648217778374596,
        18140808584162437045,
        3502136377186507090,
        4607134693582048621,
        13936949880580583914,
        2596788772608678076,
        1796713826239012327,
        4260582063129126187,
        5748801488885906379,
        16694615442676144004,
        3887305188180051618,
        2550350767205712363,
        18068173339311621649,
        12758173303022710234,
        9740862123276893551,
        5102010307820377604,
        10942936333209708005,
        18421941488948996650,
        11388637231777062032,
        9127668535392226249,
        6469837987038520357,
        14193922743168722337,
        8879463076013852218,
        1299536255974471810,
        11657123384643255381,
        7261659951101262301,
        5894363505946510117,
        4245862285394351605,
        5678371230350087802,
        17178046578086915046,
        5767351492432132301,
        2765778331274782081,
        5032104048099942935,
        17629257268172754881,
        3538476034090404567,
        9059696921132002704,
        10148267678588608063,
        7431557905485244392,
        17701219837659593435,
        11179519028472592747,
        4072852058212419917,
        6998722206646617171,
        4881588377784087158,
        8942392135657118187,
        12450098984983403167,
        4877380715364130438,
        7442425351254657978,
        16893368119062256892,
        4133190116578139434,
        14033231224823505685,
        17690698791038106309,
        69885719555671168,
        5451291976155541960,
        7641909931651695485,
        14034042004808926588,
        12720147227862124510,
        12957030947591398482,
        12124026692922785247,
        4883408107294185147,
        3308537627824539756,
        4406371143811003270,
        17321810257306281660,
        5243104860854009275,
        16431126877381187748,
        9754896554997520777,
        6749101884247660127,
        6263295512058358448,
        8583119091158463486,
        5555416137611535608,
        9162144452187531823,
        15840624178481981270,
        6187472780701357377,
        3164204801472770089,
        9460961259991884760,
        12198117330023239045,
        9445542084090344416,
        6826521970103491663,
        6996442717256261847,
        7363775912789871847,
        1821616883363314737,
        13718695280220668528,
        9265991471759860428,
        15403328465978445840,
        10435756695146507564,
        9134138337791325500,
        17595820721307989341,
        10837897611871188641,
        3628300610284347069,
        6056782082517998847,
        11889121953261384595,
        15658848261681587671,
        760281128415892566,
        9901843341271410215,
        17997287540098693599,
        11471174400256577350,
        13773435664609284401,
        224645455536727129,
        3248370257584471677,
        3402146064818516495,
        15419892046652096835,
        6817408051904455983,
        13462567452174648864,
        3865219578658340818,
        6401692728490725482,
        12241061753482985847,
        18093898726091231688,
        2085695270753393724,
        1486152210073874278,
        10962392580868374592,
        18233050379629714175,
        15440497426454102854,
        17029248778709090701,
        11943840019902256468,
        1838494147415724342,
        10624384750477063994,
        2802400104447544796,
        14482700272009891532,
        7488752288284248623,
        16689053330367821624,
        8122258539396763672,
        9378594940099937779,
        1953829770254669263,
        9095248272317331200,
        8866691394084862202,
        13201017990930506125,
        5874101266603325988,
        12758355031414108476,
        10906401543018450367,
        7070115242582431545,
        8015463853343573521,
        6337419761897564053,
        8245569799737529081,
        2143044650526073628,
        10001469296387640338,
        11129612211343148678,
        9846592166231801208,
        3293328206977069772,
        8271586651454757561,
        2516303987084978983,
        18226852422794010543,
        15105013013529789613,
        18154225228636318154,
        2013990863661732083,
        1784747160358384741,
        12567581004427688912,
        12324983884662973603,
        4935897487517618842,
        14994876132441559883,
        3460392282827044973,
        178151701108469307,
        14330685158880278679,
        17005759398710652579,
        12918079073431941444,
        14737890013936547836,
        13116140709796209375,
        4406659418908136238,
        12427086661031458855,
        7492548499951595477,
        16111633531233494957,
        6547469955282364389,
        10562100669850668222,
        15418878616411295006,
        4956757255402949102,
        8013302230254643702,
        528640010877453099,
        14207088120532813066,
        16751882716911936366,
        7949419147113034512,
        1638837693666608472,
        13878268226712426549,
        8273407385523164786,
        11509174550235804944,
        15966833959669295468,
        16501367189938100697,
        18237618137386130737,
        12449896376279048977,
        18110666911456042806,
        3193062725115205782,
        6188346541223558421,
        8058450572610971381,
        6102995183728564982,
        1217219394955358461,
        4696899915064498873,
        3897448246350199671,
        12662172956137287871,
        6610124623177066401,
        17355303367288349560,
        2046855789488294262,
        3211014904560014361,
        17115241076245460723,
        15298358927000333664,
        9182539314691893135,
        5164209594070862585,
        9062044958688820752,
        11123512834458221854,
        2659061244791686400,
        6806035717656270289,
        1027290028815967628,
        9405947735639936936,
        2540349601344589130,
        6895259978837037548,
        8904474026325226434,
        12273615493885622366,
        8677764880724506780,
        6002959367751808149,
        13123965742105252412,
        11735284964695785299,
        2417001610052848193,
        16769279424767143997,
        7098399088498274096,
        9910748236374366737,
        11111297830299095939,
        13640041663283902885,
        15915174780574330612,
        10929481089808875146,
        4581632826936763294,
        8581099749824253588,
        12645736569302882821,
        7359153121066395683,
        114042912187761457,
        8021867264848320576,
        15492861144941287563,
        17574065125533105845,
        16374369432221228355,
        10459169993431587286,
        16366034776295832243,
        2627084718398353897,
        18099240392705942111,
        4326916362018195403,
        6552915731272193737,
        4273219172539573763,
        12682979794809012349,
        2630341686377453497,
        2883112777041844122,
        17280553753834600520,
        16523493292363775458,
        4645653082909935710,
        10857150457399097857,
        16240900012104288727,
        13936061320958754124,
        7489056125188749067,
        3240618672663587347,
        4473897013556224538,
        15959787311013455542,
        6807910945586226902,
        9410336371061067189,
        6906088553149840730,
        5605158148442495482,
        884531864210940213,
        1872297304786275835,
        313522309067627421,
        17334018801756959340,
        1675436862731542012,
        8842978529412038193,
        16491099153488944340,
        3255602365380029669,
        4091823593236384660,
        9059086866751700824,
        18074592049968473162,
        8939611511088706324,
        2901954556715326677,
        3168063820638083277,
        9642457237234984168,
        12973730047202029231,
        12383420012085361069,
        11980461635855055601,
        815939574328079776,
        14011981531029282088,
        3819777392902083260,
        3664266433302366398,
        3842827072078938508,
        11680613555445372946,
        12778807841055009071,
        12110239716506060424,
        10455001326219377212,
        14037063373827903372,
        7343791660755545799,
        12328389593346708858,
        7199828106464912509,
        4858835493732869191,
        4210694898816644452,
        5207062729711049186,
        6686010191777285325,
        4765116176650914771,
        4260203503346398077,
        1759950825465072730,
        5230132100642234984,
        14615940487884510813,
        17309789486063144027,
        7278701880222450078,
        10589604955548805402,
        13389545410598327517,
        8048423454310044609,
        10174798177374038317,
        11695819581512336602,
        10798166591000354836,
        10103293254150868119,
        157182158726905133,
        1338674327495139085,
        4359740350600884386,
        3252895658818469796,
        10919494111178843097,
        4907614062557818120,
        4955680854509122805,
        11945192504152771627,
        7975797138321745283,
        6605506887556783622,
        6135553468556837256,
        12830560358150860589,
        3963138074064936645,
        17071621175761233015,
        16942996036353652149,
        18009074714271793604,
        1722999483884168113,
        12926214689279125494,
        13895491362136161766,
        2282857362875456337,
        9142994644696487964,
        9268762722221367534,
        10573580827711251601,
        4334519573500535534,
        10883875644989605426,
        7004415645651661648,
        10027237496943886389,
        13381566800231151857,
        17337229971572030119,
        9544236017718236057,
        16499558730912147790,
        16696764467460841050,
        12783153673325726862,
        4801788834514363554,
        9769596680827455135,
        2052933989795058774,
        14012190388893162910,
        2328488528431981564,
        6422943144920158158,
        3344277493662982427,
        17332086124044133390,
        6603250179050887938,
        2729973167059384364,
        10215535361886902702,
        12469382708903994601,
        1202997051177792035,
        3047394247674395800,
        2411456312940888474,
        4837088057329031859,
        13226164937828712217,
        13151155632897757600,
        11689971584362603020,
        14721057194516503031,
        3506623688955464241,
        1023806484371714024,
        616780495049061260,
        5550447694830386961,
        11303599457452379703,
        4026477090451957914,
        12426562795004232982,
        15911784896145106699,
        4007234390509453717,
        10984488057956831607,
        10127069533632441573,
        17026212809323057241,
        9318803717581063712,
        14225402896737171843,
        15831782058852344212,
        14685942491342656916,
        11353950141420870530,
        6899840258039698024,
        16365793951924111762,
        11054065770384370104,
        12313481690905923958,
        11429182630674761557,
        18037807349797853208,
        17855698087768272616,
        9322152574069274970,
        4942329044664759235,
        7995065847246945033,
        17234608471288129109,
        8540482286126463,
        14042109278380290397,
        4245459658137149297,
        8630372384706772616,
        16951967420255591896,
        6140983990759814658,
        15799001042328537181,
        293899216979442252,
        18375466344090472198,
        5023361203279264131,
        2158584219052773880,
        2430714777079983012,
        18439679719922027552,
        8247095638003041692,
        8700129222990386425,
        9065370516758531054,
        7621645239059490705,
        8198370193753546038,
        2570516211974541846,
        17033362784799911757,
        16322248788609588904,
        9688378877889012051,
        17078622631971263745,
        14000745586739559896,
        10022578092954937154,
        5445108204439341684,
        18386610455808271244,
        5092210157433941468,
        6213448264919356969,
        4883013329288095834,
        5478996930332110750,
        8026855029996672366,
        16024447858887376249,
        12067879178821404087,
        11518543008940438069,
        3823610091713721972,
        18069601801912264214,
        11258822797166212826,
        13964841203412084177,
        9414776283754541624,
        10293058710824631389,
        11948877430367495415,
        8513392542107963113,
        6296507345123212825,
        6477999943576666411,
        9572404293136150991,
        5100835030655003210,
        13325655857556084165,
        16324080076960251181,
        8879608339499901790,
        18301788022265813222,
        4530918631858474410,
        7355857660645482071,
        3649239901919106124,
        12889973024998083984,
        11695005857389286582,
        3475833048468156819,
        995729836358583730,
        12325666763829458288,
        14110440261397920019,
        13769832493884639171,
        15593753283105934190,
        7109324739675458351,
        5421147768472920505,
        4412734743279888925,
        11138246548885387220,
        14351840348441253310,
        3556324819053975644,
        464172005362873872,
        6330105154140333710,
        199716684476189301,
        30491931320003660,
        12784214407370490288,
        15867394285597021458,
        15049472685881722845,
        10832317644389672128,
        14180787391549176759,
        6195021812685555238,
        7097651423104249027,
        14212466040470804288,
        933065029493216032,
        340700391640325051,
        5222909044979876332,
        12662368682228247352,
        2784558090880586519,
        11524226622572698014,
        7185977984788112517,
        10872431215518209444,
        13748253290972801025,
        5038089701011291997,
        7535346948699029838,
        13797006813111000874,
        4892711692291451444,
        6793466025450692149,
        12784267157149526257,
        10296537403905285004,
        2323577800704132951,
        5312137140467217143,
        6430484854861543636,
        6338831277270911406,
        3380263422470065531,
        2021125064519606784,
        6156495890069554972,
        11783498133679281022,
        15578447545577779722,
        15814546899991104192,
        13941553944244468050,
        13634474865486767598,
        1056230272770991969,
        6544470833237086052,
        13788795813455716701,
        7695789694537545607,
        8392131987884965107,
        4446829805611351645,
        5546547554488839716,
        12941176365951974041,
        11083456017770653320,
        14243587238307940226,
        4340314402890900681,
        11706773788443852730,
        4344412554812442687,
        11193626359760223528,
        4305913696407512521,
        152872300974481317,
        17563972177873767735,
        6689134738611919649,
        3781413393308080648,
        8321615330761631580,
        14197467496901526225,
        14546626676044239226,
        16456266795492152384,
        11928059952577597101,
        3835042046532738036,
        13754911224713375253,
        7514650130698543676,
        8858336584435716580,
        14887091898219032624,
        16911851326374546737,
        15781280213878900978,
        13174567334832245983,
        12698715736553212443,
        482862054634250522,
        11881655003769883900,
        13759377508601776198,
        1380626582194380651,
        12709962724490385600,
        11830407677567487487,
        4707511235470141279,
        8988833435496096196,
        12849153210950239984,
        6129552893611029481,
        10992680122519594516,
        11411390234302369473,
        12356274253287847627,
        8358264938578134057,
        2779498090106587161,
        18154716656404340371,
        6939980780951404838,
        11644899556767279317,
        6503403399166403612,
        1788328284304710501,
        3402885645044344883,
        6160850982285943593,
        1361188868372704452,
        5295494278668069110,
        3856033404817891402,
        5165875819581549636,
        5432481755911013545,
        6887089866491061103,
        12248786783098658185,
        1872380111306967890,
        10371211044437237288,
        3640308804221372831,
        11707751977394310973,
        2441379059837646171,
        17261873795536786450,
        4576230298744928057,
        13439891433013201618,
        13066055068615706211,
        5023766490253071135,
        4737378710842155527,
        14130528255898995994,
        9297196539004306162,
        10213190959948266193,
        10762620391958680674,
        15631352907504559989,
    ];
    let ys = vec![
        15245088662193948010,
        854969528224537163,
        192457876290468361,
        3156774054099849881,
        10102117358735393641,
        13923135497401538045,
        15603007686998930972,
        3707765480829539463,
        1075990372015045994,
        4440028045035707188,
        779932550205535682,
        13284596850012603887,
        13447370325749987403,
        10657005451799608034,
        17344058779081327933,
        1801131630646010099,
        17879455113972297046,
        1049662270419803525,
        17887003202529550415,
        13730724178286439296,
        3086493866184691051,
        7455503161286080904,
        14945249663072669446,
        7413071270018261565,
        8165098975144402988,
        15667870805615006559,
        4534237642686726425,
        5675059133984408369,
        13542693529471369730,
        4650690134857994243,
        10593876026982724440,
        8719234160809710444,
        7340192483727047710,
        2225660849988538666,
        3260628781823840386,
        14784063213821786553,
        13478324037708856111,
        6239844587086244103,
        14508626048519473050,
        11443816492520902359,
        7084448144752764341,
        11673478635762496725,
        13444020463604694513,
        1798574113181758005,
        15195278749704748030,
        3490272214933312037,
        15632500462832370824,
        9808665338648603851,
        6377980234800091876,
        11306384233660763805,
        6392788317448223882,
        8005181869701567455,
        4601526777105113530,
        9348184476999479133,
        16105441815997897842,
        15373735633778437011,
        11733794529384137433,
        769246272107807645,
        2922899274256775805,
        16218486247871807873,
        10650657974127272786,
        579665301817927565,
        6403006378940431337,
        10150254532952843560,
        3736822004545760197,
        10244207440138560761,
        16631379436671010056,
        17418302422321190629,
        4844439457855539440,
        9662799133272397874,
        11622100630061039998,
        11017257064923257696,
        14025546287952884200,
        1170766120552674008,
        4852413824670160293,
        18019298735978800767,
        14042374992041286164,
        6103187929964524269,
        5988592592688695870,
        5579172720281387479,
        10738878044274955012,
        8401646271610146442,
        12016061916593958227,
        14752402557741497038,
        5053283107906893264,
        12910662726197463795,
        787526459034857809,
        10304827788120361107,
        8387521101013404665,
        6030209567663971422,
        7511028869236306454,
        11105170944119024313,
        2911699195421772292,
        11710398806568443147,
        7599646386487625804,
        2146501359265516686,
        1193294087739295886,
        16419769173966961854,
        14779980297792837632,
        6286361066120350249,
        8246126699673376536,
        2339493649448723726,
        12383521129608538925,
        17459816050942292574,
        7213741082075285427,
        14702683527305456088,
        17849030573001874153,
        3273901152373442943,
        10086273715179643444,
        14351251935054659627,
        3067622597087477151,
        4241957707372911307,
        16686513037697490920,
        1503886102490162470,
        4222986769290077389,
        17209928444872897872,
        10064374817012298812,
        1391022681726221923,
        3482099619102309134,
        151151415131464647,
        5477310851692317777,
        8185741896741403527,
        12297179519749775078,
        6980896315258250234,
        5491311995173541969,
        10908311176531272611,
        15140263006374103771,
        16292302828281485620,
        13488663273854028028,
        17078235461511918753,
        523009743565281503,
        11105648925812514991,
        13827146014280242829,
    ];
    let scratch = vec![0; 1444];
    let out_after = vec![
        7119213209737455664,
        8035111372284977647,
        4572010150225077091,
        9836693742249819546,
        11913987255489332987,
        9795418934522178134,
        12039823881052227957,
        1703597587030553464,
        17173440207558913559,
        9133942906034722304,
        9892190423857277000,
        11450449925285493571,
        8229203118528410575,
        18272483595874326782,
        12286453845032957559,
        17397683281593428252,
        17307726796177375265,
        5885130845879194191,
        1645176018776340060,
        7680193366109147043,
        16734119958955208667,
        6536578624341273644,
        5770440598266592917,
        843542653879105004,
        7846103239972868550,
        17767498093293336141,
        8852927397266500772,
        2375241440134626999,
        4434060166952587700,
        2761540810749122567,
        1453459469045559424,
        12942116308370467698,
        5506016901067697399,
        11535214645442000706,
        15126978367780356744,
        12154399742028506377,
        496043512841547027,
        17912935272640820199,
        10986022757903652151,
        6556183826080645558,
        14546786726747802513,
        12005748109900862598,
        7028659269647722317,
        7445941872312834859,
        17378556553372415985,
        6579722616703715705,
        15110932971155985188,
        13357716021096701878,
        14805225292079398749,
        4846395190680470669,
        5496704586153785876,
        13902332896674228944,
        2782564501691101676,
        12874276640173358887,
        16083587358039033190,
        18355037119147466217,
        17905645348986250355,
        17336266670070672432,
        7329875537940689357,
        15143929848768191816,
        2335192162346001268,
        3675321604843848800,
        18055470666128734882,
        7825904415481215047,
        5109358240684702569,
        14285416470839730663,
        5712671120250540046,
        1447875149083639731,
        16825523458715974110,
        435887853762049989,
        6289440856649262026,
        5079567923272969509,
        11992232716208537238,
        13597285185096130255,
        11829047305733652617,
        4311086504067261381,
        15175721011032302019,
        5062112127396725677,
        5556662484107594780,
        16961425301235550719,
        7249744124354557124,
        10415169108908960370,
        10792331692568535985,
        14481109731941072963,
        3493798658848957833,
        3820132639380218489,
        16270989634898866497,
        12612959379075825823,
        2261231936675595340,
        5397727033090527090,
        1595530071661307000,
        8721991197262817968,
        2097281716258616855,
        9720159116732069000,
        13847028604290243745,
        1259097204603535157,
        4488418560755904013,
        6367333815591827979,
        1110836537797269745,
        737322139420954828,
        10719862516375734296,
        17641235722877078710,
        4007603116973007245,
        8186031591789355966,
        824809602592127358,
        15459023162084506830,
        2781779707947579519,
        5340603447447048495,
        10487772282235399213,
        139590844844775293,
        8448523529988919651,
        11294430762236913551,
        7417787798712521690,
        7898687579828307114,
        460432324038279684,
        17287858692967790320,
        1003171402208274758,
        12432863151610498290,
        1772583694811023620,
        2398714257155470903,
        14638551745572956179,
        2453888832037177615,
        17965720127679021564,
        3482752704874523706,
        9460983625185291928,
        7861173646556463420,
        2092158584793655622,
        7860343108590151278,
        5798668224892394938,
        14344358780928405264,
        8930780312248882738,
        2041974908151755313,
        5267057435060043630,
        16349241761951706605,
        7937085583298782596,
        1533874153792017121,
        8977390985744119243,
        8213952522673826258,
        5985476232972805345,
        1767819075525805690,
        7890652703767299436,
        7833570395707421953,
        16483497150383186636,
        3111844668913506626,
        10592215978440849992,
        15760470899502034545,
        14841199295755801240,
        13457358926996905771,
        8495921566729048945,
        12384251008552336501,
        14611927430049516374,
        10145411926842313555,
        13313983588964854484,
        6522555254793521962,
        14886168792472123947,
        16563223991574184817,
        11970636068147089860,
        487573490069849866,
        14517767327925834099,
        358889874180177533,
        2722904686908914583,
        9331922038722423702,
        10427207646299323058,
        4058944134252549339,
        2943158119917296684,
        10957133362022867886,
        11807644687289257630,
        14152770343532942991,
        644765569764633457,
        12835933538365183555,
        3489836693155058202,
        7930565595074376717,
        10663397566559525483,
        10415977712042153468,
        10629891694308728614,
        17218729211483121702,
        12882631064819497692,
        11823247409777216976,
        4429760090149085341,
        4992836119218612253,
        16968182235411645979,
        2833341723517409131,
        9437468414857756700,
        9342501551191707874,
        5401538590112566847,
        11565576817184943683,
        17418902749314320121,
        15515057935262510,
        13023198767155479105,
        12322345548326767749,
        9805678358507392118,
        15196802879249030007,
        8590456626380559194,
        12181129450616746226,
        4662360147888268775,
        7743459026687301615,
        12227881705402238521,
        4104871394210118957,
        2982631795273577607,
        17405629239503027027,
        8575042458975896270,
        5499669298608315445,
        455410535869802297,
        7801072276748452974,
        10258654969684304036,
        5019332302330169406,
        13851706526542678368,
        14388718431838510370,
        6994627771674318027,
        5883188522176688608,
        6953100454027368107,
        16730449049154094699,
        4479760571164126224,
        2442817149002290432,
        2594209501779693152,
        17000985552959046155,
        13392630901869088171,
        4351699921480478565,
        830530327244063521,
        6175340290035796626,
        11478838337470764581,
        4282543001913023167,
        9749956107550685821,
        2694451472405341333,
        14159005131955878971,
        188623324081031138,
        2113957709622881014,
        13346278264743742220,
        1490312624045928630,
        7257319265988714790,
        12897840546280054308,
        10261220359148798170,
        11623514965826520552,
        804176158748924112,
        17254294564975652336,
        14961613837993764505,
        17022609814041542140,
        5050795566934959858,
        3133859061844734723,
        16471861622774443737,
        12873904550161604219,
        8955720228538638459,
        6705947195280088455,
        3312252979433304494,
        13064308724359909641,
        17877454778920390520,
        13974626528815595051,
        5830057811107410067,
        1680557938316663873,
        5360522541537691866,
        16002353064982324832,
        10193786265901148495,
        5965825498900060524,
        14004637605477595175,
        13444980433515313059,
        17123381832713041503,
        395654489392928755,
        17905210858022857309,
        11695619457011399410,
        2050293660264041357,
        13214005508111947011,
        13721733314071779620,
        6957322551477640013,
        17200941870372170988,
        10377268746838169434,
        17697877660887283146,
        13909454659468606094,
        12263365692987306251,
        444868280971136448,
        2249487486097442707,
        10287367344673627295,
        15227830943213591291,
        11384605430871297252,
        18114267789275369190,
        9609419984349670064,
        7173216608425317330,
        3785723985326055000,
        2525275194754047614,
        13535151049834722889,
        17273215075095328812,
        9379065185520973134,
        13156080046058576550,
        13809311185533464020,
        9698193583022285357,
        11293045682332858022,
        3789086099334034431,
        17423609786192009535,
        8700938259391751384,
        6754612211313466065,
        15592171130416925401,
        16281409111498332713,
        3330243176102830812,
        12431102122589837326,
        16006104650687086798,
        5881832849234936462,
        9558494019188617372,
        5910373903796225502,
        7012489078165397708,
        1678461096818873552,
        14039349728684147747,
        16767452777528123993,
        10223391806015696982,
        15346818030164147356,
        1162766693101697053,
        18234451668077951680,
        8347242572094928214,
        6666731739952625453,
        1062103725054959015,
        2916817885872081012,
        1252775340062153472,
        16701352066718298728,
        9660144340038428104,
        14959287155729434404,
        3349821657014683971,
        14124329647242713467,
        16768224457728633355,
        15245206340382728113,
        13349975818727705647,
        7060168455288126345,
        14358009543123557047,
        1671493412977635062,
        13147000836819401629,
        18412981554162763098,
        11420833773021925355,
        4462121868113237202,
        3673613052681393780,
        7933932278185611848,
        2994094088837084031,
        617851389487072906,
        936087254346174626,
        6282395350939998602,
        5564739907135838574,
        357676340385459789,
        4488114251655788009,
        15267875158424192348,
        2784291869179885450,
        11312568718856999737,
        8569642291350942890,
        2238202569818012692,
        11656862683686826121,
        15262363620109289295,
        16524990113573459437,
        5562790604256723201,
        3345369486980494832,
        12006274934570630789,
        9946114845665174629,
        13671323181231836463,
        3970220369626629149,
        7776022796090455195,
        9590755788237311534,
        8142856981292265441,
        321465156536801162,
        5953069648150855351,
        16986574134926902837,
        9332812118546582409,
        10453305700229109136,
        6745273322860959577,
        2992371413743712642,
        16607737435713483528,
        6982491039985356345,
        10635374366254201081,
        5489787617426825481,
        16512907085669471327,
        12008394332273820459,
        13681713057282914887,
        1825484281011599499,
        10265338194463676800,
        231895184071445598,
        8684485436743230364,
        18314390632816293564,
        395173997200045924,
        3458959778923434284,
        14875618389446705178,
        2026228713879608348,
        8096896632351243884,
        11898068926472226654,
        11401424330286284371,
        11388179279009736427,
        8567780769863850951,
        5420955600339252740,
        9557718016358557523,
        10293539895922743855,
        10709104391440822839,
        15398390488930753996,
        11609873443226591907,
        3799303761026674488,
        14827504231551333743,
        5857678196927724111,
        8450122285857845619,
        6120869104368170858,
        13208767594813061501,
        17440504615103435089,
        1352314430040358495,
        4594662627540876693,
        16103658665190650049,
        4604995467068533697,
        1965756000963933588,
        11590357129340578915,
        10706020607921767648,
        435959974903581847,
        8114156514383604689,
        150635546028022237,
        2046711193518988780,
        9302159929442525204,
        4102862072533955538,
        10755931112512381521,
        13759570670120566522,
        11886492672468308648,
        18368564319937059130,
        9170256735689590352,
        89744806848865555,
        17004092425291241458,
        13341437623654656084,
        9602103451691077650,
        805686450066544356,
        2830839197815761419,
        1956329411504418821,
        1856751313451126046,
        8360952867730129523,
        2655094640585883682,
        1812351508323318765,
        5466452692324458722,
        2317993471554330319,
        8386109932957812768,
        191643434261986870,
        8672383672749866782,
        2088248899380502026,
        7296610571351867175,
        13755695345060796205,
        2999982423435890541,
        4129519642079256449,
        12622156276462967022,
        7111496398765881581,
        14480787311565160150,
        3469939266209117746,
        7438701329152033623,
        11629269482462741856,
        4885777542024810464,
        10705129561986474549,
        12704646697057875,
        6458480273092609348,
        6552269679491575287,
        16764409845541378365,
        5112781772516022587,
        8008989787537536761,
        1103584530948037829,
        16382841334779146359,
        10554805089071823936,
        5435260964051060833,
        9791508728754514420,
        490382068908134805,
        17435419070710567937,
        14195398757764953304,
        2963888499740999291,
        4662041165840469949,
        6744621662455431846,
        13427204268640870135,
        3887747288146601313,
        15443876414310792510,
        14209765039523837918,
        15685162874875886369,
        17718873723062454563,
        3899085415479325935,
        8112448753673506965,
        9870640447409364221,
        11832870733499002257,
        12563697741369320162,
        7101757806260064829,
        8836333095435223130,
        2301074177341928226,
        8276824997793946999,
        2827834280767843126,
        11204234843351418659,
        14057032714422307544,
        16637779355549534512,
        17293043019105638911,
        5235444579393367735,
        7875158171121328348,
        12032932633735348172,
        17577724652120330660,
        7795364456390606477,
        4033162617687866088,
        1544683855253756180,
        12172662201233721356,
        3756704469121795835,
        15900453600187755478,
        3111611615821110222,
        12067333759598666114,
        2460839148177982607,
        7102750419395327210,
        11545587889066437759,
        13014356072259624231,
        9055277810304247492,
        14625948105355344074,
        14619092238615835856,
        4627724365506319628,
        12192367475393288119,
        4660777525583106578,
        10602596814120768352,
        15235390391666982533,
        6262394583518039046,
        11215546472975813276,
        17221191340393185135,
        2101601180397124797,
        5801945926643183687,
        13279838326750802748,
        9760225764448182588,
        10638379244001938587,
        14689414070605049956,
        9642847410658476483,
        9252181208713310128,
        10194326651074197381,
        4194873578760608787,
        15948390572816263861,
        11413972117606094823,
        5666682562527317380,
        4558761157644127095,
        16955166507135813953,
        17165455630592879759,
        6709237464973926762,
        8519876379139259581,
        16017543326401035900,
        13443243112246019386,
        15985758849225110622,
        6635201996777679527,
        810401065572784823,
        3192312066161014982,
        9614569995908311868,
        6150822398486473654,
        10813585281758350005,
        8073858541355843438,
        18109256134774887184,
        3173606297852832180,
        1697871109011675466,
        18299820979700369122,
        3429453844247078700,
        5880410106220533795,
        1902086387150156227,
        1095558516266363748,
        7401629960716178240,
        10676693679576960757,
        6803369895878427000,
        679584762058495726,
        8862820141296633751,
        17191945777921342135,
        8948907246409262383,
        3201694424452878897,
        13419812787573406307,
        1123422467792803886,
        16829507079638403272,
        231077225656705538,
        1743801854754216927,
        2267894415989934871,
        10485638453110401044,
        14128191714015568951,
        3134003464552465993,
        16422231357711592145,
        10265419470005494808,
        608502939641364027,
        9134620013637384764,
        12129300312054467857,
        18363959585640954741,
        9300192354341214544,
        10785267438521530991,
        4936458668110384357,
        17991939764954279170,
        1611386959702759931,
        13765557038981607892,
        9945874597041578052,
        4156215181237754094,
        18063177967743727852,
        15711037733871659565,
        18074471117735370517,
        4165544053496883142,
        5279695231734291715,
        1544829019313055335,
        10641384862840576547,
        17779765763310188495,
        5626860848734533473,
        10939014392735308139,
        499526074720537175,
        6841635225707590127,
        8751796502513481516,
        11336340410331983928,
        8161799980755225229,
        3303335824150755556,
        187196914320055288,
        4058076549799940817,
        15401421223440460347,
        10934724927412220331,
        4611229640484146507,
        4222099538065534859,
        17723869573083320478,
        4326485073257732271,
        14233319419690524901,
        10105594311301668570,
        13674225839779959556,
        9223247539431781352,
        5207133412458563780,
        2543064725421193686,
        16112712767696009427,
        14180139311525005755,
        8623263245810438804,
        9872551252324828981,
        7186327511028170781,
        15405568215949189408,
        648741987472856599,
        15504353064913393130,
        9477833781488603469,
        15265929637105201324,
        14321785262909727449,
        5989145253796307072,
        4029928268282443946,
        214922776543851469,
        14661242500471677738,
        17649450017688656343,
        8833974540349191087,
        13816848681848193227,
        9357698275516764935,
        4896085333994455104,
        15704289681227768777,
        16072211553259947696,
        3696086675995567997,
        2705327250235057879,
        3940607606521307638,
        1904439667923426432,
        5211873495099253275,
        12248061263800319556,
        5068931489527810664,
        3181822810361134627,
        2893763819599633264,
        17022013858675764406,
        14432435821204701872,
        5652574963789809237,
        2389493332555305833,
        7138156757153353567,
        11060288640704436502,
        16742145355448544339,
        13832335420610765016,
        13868994572073192755,
        7515903319542781099,
        6444421916061246995,
        11587739604890358174,
        6732571924439013884,
        11724575869028146429,
        12250459832157882902,
        3016621587617492227,
        9032739925629140208,
        17557103950413855460,
        13601852701212599067,
        10511208766396769010,
        1798260311095193927,
        5693423254942127954,
        12655656726468149287,
        11019197760486093174,
        12053338214040694646,
        18408373830237784798,
        6067223765497148246,
        11553996073859513969,
        4236552940838351761,
        9458987771514077962,
        11856103773548100132,
        12956542515015205037,
        4644161276287406073,
        8685301764457396658,
        1616507854541499544,
        10201320335307975884,
        4576763754089281023,
        2461338218102810280,
        91262760005101889,
        11335499049722647439,
        4878593686343800958,
        12877266813141124945,
        11027535651547021298,
        5250333616946137743,
        7088081721141062141,
        14298861153503504925,
        7147145447148416343,
        17047545950712643349,
        4493827337401588739,
        12928986855103603232,
        15542818123557007044,
        13140774562703256605,
        3866700096503647014,
        11880459985577020267,
        9892678920934094494,
        7884199841722717749,
        15763760571691417193,
        10614999011319354481,
        12234066605168430720,
        13097736774835846982,
        11019050032839105206,
        127993841633501445,
        18442075166379932728,
        5173423558140830113,
        5040877753307545604,
        17492079016333825987,
        1593793193144791850,
        1051352355195775735,
        5943763991772466352,
        13308051483591590190,
        9343693271988203192,
        3520113272999099435,
        13156437544558533124,
        4628664430619092695,
        13839519774612268130,
        1561524063476792872,
        1786105529617874762,
        10743318550189039818,
        4780466308508262376,
        16286344021354707563,
        7706876683201466759,
        6056690298161881679,
        10055051256791782547,
        2985438843283121106,
        5662611066107180778,
        11762504057786722681,
        6492163523688561469,
        11903314370703515089,
        6639143404964802752,
        15352864297227042626,
        6467331344424294098,
        2513138096890065757,
        18252010886576525300,
        2137309836787405465,
        5062748182344476458,
        11543605570449724492,
        8967616081750626506,
        14081282030384505465,
        14030488276049836901,
        10391269999968368055,
        1394499028419468024,
        15566682361173996820,
        4824266726532780984,
        10757268905789656792,
        16462615027839033267,
        3588952369216858747,
        7608658025855812414,
        17838642938949526141,
        94781055844175262,
        711508162709410434,
        16198763120066216543,
        14522827050618946105,
        8827834881271403758,
        14961924349064399313,
        14406825157858343587,
        10665706975132276032,
        584919077692434094,
        12685151994185398186,
        13214723590065181555,
        13213787529887690386,
        8702815823276144804,
        11800457387620589360,
        17199643293763420877,
        11515884665346482491,
        13888785899717119262,
        3507252794824964582,
        4603659836413814919,
        4686748937472750550,
        12421832631783901099,
        3359573926155290420,
        8446017905393844760,
        2262184677731317052,
        13013248939510347462,
        14572693118580589660,
        11022830998326129411,
        8408924655992625371,
        10410179516308402041,
        6729141891150201635,
        10466696767386859740,
        2677788096731405850,
        8361098508510730897,
        1435994109558943177,
        10128073597738606818,
        921651131688578288,
        3784288897383787482,
        2528827532726381824,
        2745271865598231885,
        4303509657664184356,
        8598151439188559180,
        7339838340151008759,
        4102212049950464432,
        12029559280219172530,
        8134545883285853776,
        1905850669581557220,
        16936295454560454463,
        3680651425193647974,
        13599577612277546424,
        15046713416606033193,
        456361953791851675,
        4752112709649208116,
        16115819506587034188,
        2144813052901824207,
        17046622154581665407,
        2876106397373513535,
        13003327799466428462,
        16437033639571166321,
        2399609620924500287,
        6243590631410196409,
        533278795730307090,
        1033111886262872863,
        4866574245728597909,
        2464030264651522958,
        13205453272650124362,
        11723186208228000543,
        12969372141223026578,
        13167498178415442543,
        11172031751757823858,
        13257370093102747268,
        3420723884940420444,
        1598261708381549908,
        4391211331742607590,
        17534127798353752916,
        7341206993048506664,
        14415020221961337361,
        2046719475561107676,
        4259820090742316823,
        9276901061275297500,
        4961643293623327681,
        17911768139050064538,
        7558076127984989238,
        568046345613284638,
        6669214817759616213,
        13786197531743943161,
        4249924081611420922,
        3529848342623085628,
        5686448815512694800,
        2430075147871613846,
        1061366453609214842,
        11667688802723514202,
        10816922529577173425,
        15297819755158335361,
        12893049998502519066,
        8535913883749763696,
        6339822152676352172,
        15201150192746395949,
        6947773417864595038,
        1035062681571489613,
        5378407968343096021,
        7821838471743655524,
        8160693950261852840,
        3806489823381045221,
        9110537314944891991,
        6678462809405704107,
        13148862938001788174,
        18343376155927784359,
        1347615916836827854,
        1983837968641288273,
        4808698262452924116,
        16203540206877084005,
        3735610071301703267,
        3879614205585090263,
        14396127625458802513,
        12971705508774594652,
        16855551153787363016,
        13770590905219407421,
        8320920749683191847,
        12731079676051788661,
        6315049700706159729,
        5520681748122702259,
        18377945140487117991,
        3750012964925257010,
        6112975361804514180,
        7762601797879075263,
        7806585903492997505,
        11722560531200514496,
        1245601739242598527,
        9851142601372397178,
        5448093531458783000,
        6655626903986665730,
        1791030450653932131,
        4267138194183414466,
        11787121723746760335,
        8737617848190145545,
        1279353625599713991,
        16837389613968560264,
        12170971524469207688,
        5744210997797297631,
        6252807583283253984,
        11981409579764369313,
        13423106411424571122,
        13506594589042509585,
        9696374086757887423,
        12259201030407607956,
        1940425431459421890,
        6794826149146208424,
        3390019520570000181,
        15663291992287526262,
        8024326304926259059,
        15605237955532295324,
        7521900870833785280,
        17076272368081584726,
        14317742593729071792,
        6186803591370756697,
        8693074866179823680,
        4521657215447236495,
        7139214124438529536,
        11252767985352986779,
        9244757738422684376,
        9181380892380409585,
        14124397466598078340,
        14092092430731683920,
        13444150800556990844,
        4618564184049153098,
        11716810196377677056,
    ];
    test(out, 960, xs, ys, scratch, out_after);
    test(
        vec![
            14220278540957382380,
            2341486122221271577,
            7196589596013057856,
            1551746986989660725,
            10730116577084868866,
            5936722303301071470,
            10369883251536410381,
            12850276745016711948,
            5296892922384710655,
            17927383456923143639,
            4297800151447374193,
            13092886296891104150,
            4359313044481849916,
            3298387784200302647,
            16596015395483459285,
            6430047254377585078,
            6434371216959893287,
            14030068303817750616,
            17753911968053092798,
            4279608746592328433,
            4376377052237075099,
            41468300827872346,
            14769069119913127616,
            9177168551686374492,
            15140565915751014925,
            924212491380315180,
            1560999192236438812,
            18226600332889405930,
            13348576723395503060,
            3609063818274619379,
            10959876508495766635,
            1673947094507794325,
            8930147615298565568,
            1510138210058461841,
            138658670646289458,
            12458879312664552547,
            2808979850323582450,
            14034036666484023484,
            5585848372598666548,
            4257210144363692139,
            6338629395801650540,
            17487850811313620991,
            11723765031594531899,
            12058758430990318672,
            13113234745283246366,
            14258831759967032298,
            5702057171550109179,
            13097449289463197974,
            4956526745376824433,
            5705156949115651685,
            12015733702012855186,
            8669471405965585399,
            1905089081593904691,
            9375413288655317408,
            2862087246546719140,
            1438802037360108086,
            18188344038773760076,
            6710952137267346735,
            5113990331512729307,
            1249942195157804768,
            12370173123137454524,
            17270573617823502592,
            7850217070904007210,
            15008954292697138434,
            14323227552451257069,
            3071194499601430769,
            1423266339629343645,
            17719424777360227280,
            5817741038340475518,
            2288688081269683349,
            1294813101366983999,
            11187697042332288595,
            16383307749918316156,
            11964485411396601998,
            2882482593432453314,
            9642249517934314115,
            10542384929787952175,
            1685709334753615213,
            9706601754650993843,
            1704742035456300361,
            14513229339862334723,
            6682003045097239025,
            4714611199273627241,
            10432585451230614215,
            5273605414681358823,
            9969985476182536371,
            5399491654701460340,
            13433254437284849384,
            8737041519297820559,
            14151041462241779583,
            12800325041646118168,
            1054683376318581041,
            10442674392431772880,
            1301805922104023475,
            12991162900765496493,
            14882797183035693101,
            4301011085545481404,
            11250534379510854507,
            6111542722921923807,
            16925710356305375375,
            4650629800913345495,
            8040250152135495246,
            6010724734735944849,
            704649712350181160,
            12623719311045527320,
            17274359723691631952,
            12634456101984828972,
            18130410107220630156,
            11889003125474723933,
            16069412996687629414,
            14767547198599161992,
            1104256363303539555,
        ],
        112,
        vec![
            10900792384749518304,
            1752565570529908396,
            3402229115647561238,
            2472381872242532960,
            15748089475115162936,
            1958855681762413475,
            12100416912810779188,
            12256578057348862042,
            6556831077371185734,
            15914846823057329492,
            17346954154793811255,
            17566187606614467459,
            1433606366066775495,
            9089332045922722756,
            10056944581186126460,
            5324425019386643029,
            5281765195814058625,
            1449711238109407238,
            5903959110668039125,
            3336955200427408551,
            751494194154096512,
            15350321905800137137,
            12407374450431165353,
            8705815621686854350,
            18038286270431178148,
            11671842546699641930,
            9343865367071815679,
            13401838367914321671,
            18365991333043790435,
            17428290065100096976,
            6040216493892400727,
            4224515713015397505,
            16578741590625036060,
            11835373548777581169,
            18413478850867685366,
            8453265724386285209,
            5394500720474148965,
            1927463313122594080,
            4177838821929605731,
            10680620304882583021,
            180005403771618203,
            2256408572502279608,
            11718426532525535626,
            14260315287467647015,
            4035759666841010016,
            16259497729863184485,
            7772704202422133476,
            6815813069474359325,
            11207378575459431371,
            18308033894506293455,
            9875145231436590806,
            15354934628544213986,
            761822562304640527,
            7391550101325083295,
            4023926600201752832,
            922969942182092752,
            12110946035817932140,
            16574399923422896843,
            7087993004495856759,
            8299428112066197874,
            4589434828506762129,
            13978205413565566735,
            15675366647238478172,
            7819770375827015142,
            6823625407675143456,
            2042269662440457350,
            11521115322912473140,
            13703874674141705702,
            1295561690992462505,
            12464082489717915012,
            11378922861990148970,
            2076282285705918066,
            1390689690731346588,
            13670979351308531000,
            12980996477862222169,
            10496970808504864546,
            14015388605987660396,
            4171129107047347396,
            1656857204469415571,
            17492457435753920912,
            10132937897450237781,
            5065601177732655021,
            17498367701449356268,
            9552937910825811119,
            6213399497134928078,
            12865519292113075754,
            8548871019474664332,
            12973212090641168109,
            3018095992673320728,
            4102580256148037725,
            11411505815957245048,
            8044142604358855954,
            6163064685377006161,
            7676133172626151339,
            15177331097378985405,
            923128391789363540,
            8405355494789853124,
            8409013636286216842,
            17519952046647436442,
            12690425880634822079,
            7295927951214020420,
            5103171252065286692,
            4531269626482776566,
            17509133779966482098,
            16771567673323510549,
            9940518318209913958,
            2566490491730418524,
            4997841530198583881,
            11759671980624847072,
            12804335234851198898,
        ],
        vec![
            564820221219774033,
            4488711358679422475,
            10020082426042197380,
            17225157352286806558,
            5780768250525361575,
            1970180556702143116,
            5857604197270789289,
            4060596445048742789,
            4197799076012455571,
            7044577438443748571,
            9865458079653433267,
            16329626967551115891,
            4152461199188161627,
            13000775528850398936,
            7619420622350160180,
            14900279174214956337,
            1704825421557733731,
            47372161928033978,
            3056759021249434255,
            16034528189533406528,
            6435981853629992716,
            7347416955208902363,
            7867885339734871956,
            16003312811447303393,
            11973054691848315139,
            4061237791967812067,
            2991418391396596002,
            4703879799196538602,
        ],
        vec![
            17578771842269238556,
            10550562699554142644,
            8014103604728001619,
            7390436293466268454,
            7100508597220429999,
            13756539744133134331,
            3525655558627413814,
            170556856626573274,
            13733802409149293411,
            7547658189901533049,
            10109863308220110581,
            2718018624165645986,
            17221869495150324628,
            1084076247398656329,
            9986824749465272556,
            9281213789632200478,
            8263835511857229414,
            2738190782342964576,
            4727749457903038288,
            5193416305832697137,
            10922073469544468796,
            10479120898291460619,
            16531138594227806551,
            5669282286095321480,
            910652003754331469,
            7613181106845480358,
            9797364435815597385,
            2605791728953382133,
            3252270662152843680,
            14622379517499579600,
            15478463498321531331,
            11333349078754873211,
            3607338878951398292,
            13712359136800994328,
            2877919791629132004,
            16504311509495367657,
            3984067008787761258,
            12750589284505094681,
            2943531823722430709,
            13059724297333425458,
            913829495058925011,
            18148528311470701517,
            9740722521202851845,
            14187622249425297058,
            18404294373540546372,
            16126489610510541686,
            7594528449544330894,
            7307162781265364836,
            18291168119091813753,
            6207931594466286839,
            4606167418509673090,
            14995240148627089997,
            15715313103810577432,
            18105567578045566655,
            722560899028528632,
            7800517321196764697,
            0,
            11237865801974219633,
            8495979800230117621,
            9987880506353342706,
            13881397368512772448,
            16410969001089181498,
            333374346498930988,
            4965563385767496990,
            672660669687292950,
            15110765557390623369,
            6227398366445560799,
            10437883884413763417,
            186853687685224617,
            12984935070726568218,
            17363252202577108630,
            11014686580262231685,
            12860125675167553971,
            5547449898028817740,
            1954311880857527678,
            12081499992106861757,
            14088922543958348561,
            5962182563926402036,
            5741569353545905243,
            4210355652762257668,
            9053751503514703697,
            7217335315946897611,
            6001545942842673798,
            2462625304327945285,
            291604632040409357,
            2217739517798639603,
            2459684629220685427,
            3250634546851783096,
            6059950473822714693,
            582656439794855702,
            10281608630795849151,
            15786006397997472307,
            16261716592524561399,
            17594773541694900671,
            16563607893390940119,
            15505686658080216809,
            2051217090370709743,
            13908204460746584986,
            10039065459685700894,
            14729811272925582111,
            2491677743278343614,
            2841321092069312904,
            15064468426708488588,
            9844013577617278901,
            15394077255564897694,
            7846267917227712141,
            2926235763818832770,
            3614802673424810100,
            9439654670958755441,
            11228283553719081427,
            300805303577965353,
            137972366037263628,
            0,
            0,
            17236590422641137780,
            3624909720816563168,
            14760980184861256094,
            12619697833885886701,
            11158654646608400806,
            6427394341906398356,
            14871794339281852631,
            4436807681521846899,
            18179949743405593894,
            13872577160616872141,
            5825838831881338115,
            3862312932472761757,
            138044675074312990,
            15071993629914359360,
            17124765792905529105,
            3248142733680724962,
            3891075505082712037,
            6225475960510427854,
            11369706706515368571,
            11286728465632095620,
            5182849661875987731,
            11179192798752789740,
            10750517245961749782,
            9660102259642485054,
            7905348372980940366,
            6606241368966986909,
            10292241739332011027,
            3848900457088510551,
            12152591835908862357,
            4562770772987021222,
            15938089548127288011,
            9698047696083781011,
            13560645597951715331,
            7732793292629543444,
            7001973034910440318,
            409123120027429255,
            17678180108806694420,
            12698074214205994356,
            7447251798260171941,
            9757491913093219480,
            10221393982691316695,
            12294139009925614381,
            12645218559587650799,
            1569889406832824935,
            15186575789336541212,
            11156326477797897792,
            3241434575939356910,
            7753423363217428843,
            12882554975845472437,
            8367515576296379496,
            7308654739706172282,
            10357093098345630105,
            7448894655389345071,
            13033958940183436012,
            4023926600201752831,
            922969942182092752,
            0,
            0,
        ],
        vec![
            428006685349647609,
            7204145293518952443,
            11791003654963925746,
            2505792214484682253,
            8108316487989608659,
            13780196866264665329,
            9953181804399070508,
            4136674009057114789,
            16606729965278593786,
            17177157337945625561,
            4837680254021048240,
            9379658380128247416,
            3012240265336162150,
            10902131616911906952,
            4485885110776748559,
            11729287033558411069,
            3601025811236822387,
            16042927197309963407,
            15195320086232266472,
            1335053215979905641,
            13532069890206403083,
            18094969954718697842,
            11554202269641514254,
            6403207599202754602,
            5506746445690691110,
            15578759879114917443,
            8851198090240221910,
            33092233213362571,
            3901122084110515729,
            1066956798888524232,
            14634384737761094242,
            12113217349928728328,
            9311779266239300758,
            3954657017302988647,
            15532948787491930233,
            7561648003514142824,
            3533375924863968581,
            11403256543506170549,
            14720984883173782359,
            17386473882517270759,
            13904760793036335927,
            1743693703212112035,
            16080759313684757024,
            16414994735840739,
            5271412552749845691,
            14285133708577039222,
            2285218927081566054,
            4416233718641470493,
            18000037526325532907,
            9874370178595584823,
            2431098204987529914,
            16053941488194364614,
            13953671515916339883,
            2042478395473246676,
            15825222700565763229,
            215338277757125297,
            11715801345488940860,
            15276655380547065511,
            18173108185582673895,
            14702410041012894911,
            17207703913939254166,
            4048944409583435567,
            9240704600129021640,
            10746010074070008578,
            10882274053549736437,
            17502803382613798018,
            17103667474686148623,
            1210754893985321636,
            14411333195446047958,
            9505671694718631767,
            17438945960504601463,
            17181086340701682720,
            2087061119253391199,
            4309925416075845511,
            15682876991754565762,
            1083109033127246208,
            4261779269652031267,
            12103546078796301947,
            11869822949907847429,
            10776362556491885652,
            3496260740980747565,
            1952845492765004055,
            3956805099328134345,
            13449597823431522461,
            1303096111058814577,
            2380211731709650934,
            18106526914056683197,
            11484139358020791769,
            2127792824095743993,
            4065001616617210813,
            16699812364763026472,
            7039834103740457737,
            13922378224749177434,
            3842744781605852390,
            2881049947442869386,
            9197037672589549736,
            16294658006443583090,
            1354837441772061197,
            6800107739861512622,
            7157401423299319780,
            11344094968696196811,
            14476284546277236050,
            5424464793786542079,
            5963838631551795433,
            5066844411121684619,
            17308776236012135105,
            4153319859448120557,
            9810663971090332314,
            4914337127311050195,
            3998867848399474691,
            5662987571533822588,
            879647540404803095,
        ],
    );
}

fn verify_mul_low_1(out_before: &[Limb], xs: &[Limb], ys: &[Limb], out_after: &[Limb]) {
    let n = Natural::from_limbs_asc(xs) * Natural::from_limbs_asc(ys);
    let mut ns = n.into_limbs_asc();
    let len = xs.len();
    ns.resize(len, 0);
    assert_eq!(ns, &out_after[..len]);
    assert_eq!(&out_after[len..], &out_before[len..]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mul_low_same_length_basecase() {
    let test = |xs: Vec<Limb>, ys: Vec<Limb>, out_before: Vec<Limb>, out_after: Vec<Limb>| {
        let mut out = out_before.clone();
        limbs_mul_low_same_length_basecase(&mut out, &xs, &ys);
        assert_eq!(out, out_after);

        let mut out = out_before.clone();
        limbs_mul_low_same_length_basecase_alt(&mut out, &xs, &ys);
        assert_eq!(out, out_after);

        verify_mul_low_1(&out_before, &xs, &ys, &out_after);
    };
    test(vec![2], vec![3], vec![10; 3], vec![6, 10, 10]);
    test(
        vec![1; 3],
        series(1, 3),
        vec![5; 8],
        vec![1, 3, 6, 5, 5, 5, 5, 5],
    );
    test(
        vec![100, 101, 102],
        vec![102, 101, 100],
        vec![10; 7],
        vec![10200, 20402, 30605, 10, 10, 10, 10],
    );
    test(vec![u32::MAX], vec![1], vec![10; 3], vec![u32::MAX, 10, 10]);
    test(
        vec![u32::MAX],
        vec![u32::MAX],
        vec![10; 4],
        vec![1, 10, 10, 10],
    );
    test(
        vec![u32::MAX; 3],
        vec![u32::MAX; 3],
        vec![10; 6],
        vec![1, 0, 0, 10, 10, 10],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_low_same_length_basecase_fail_1() {
    let mut out = vec![10, 10, 10, 10, 10];
    limbs_mul_low_same_length_basecase(&mut out, &[6, 7], &[1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_low_same_length_basecase_fail_2() {
    let mut out = vec![10];
    limbs_mul_low_same_length_basecase(&mut out, &[6, 7], &[1, 2]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_low_same_length_basecase_fail_3() {
    let mut out = vec![10];
    limbs_mul_low_same_length_basecase(&mut out, &[], &[]);
}

#[test]
fn limbs_mul_low_same_length_basecase_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_triple_gen_var_24().test_properties_with_config(
        &config,
        |(out_before, xs, ys)| {
            let mut out = out_before.to_vec();
            limbs_mul_low_same_length_basecase(&mut out, &xs, &ys);

            let out_after = out;
            let mut out = out_before.to_vec();
            limbs_mul_low_same_length_basecase_alt(&mut out, &xs, &ys);
            assert_eq!(out, out_after);

            verify_mul_low_1(&out_before, &xs, &ys, &out_after);
        },
    );
}

fn verify_mul_low_2(xs: &[Limb], ys: &[Limb], out: &[Limb]) {
    let n = Natural::from_limbs_asc(xs) * Natural::from_limbs_asc(ys);
    let mut ns = n.into_limbs_asc();
    let len = xs.len();
    ns.resize(len, 0);
    assert_eq!(ns, out);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mul_low_same_length_divide_and_conquer() {
    let test = |xs: Vec<Limb>, ys: Vec<Limb>, out_before: Vec<Limb>, out_after: &[Limb]| {
        let len = xs.len();
        let mut out = out_before.clone();
        limbs_mul_low_same_length_divide_and_conquer_shared_scratch(&mut out, &xs, &ys);
        assert_eq!(&out[..len], out_after);

        let mut out = out_before;
        let mut scratch = vec![0; xs.len() << 1];
        limbs_mul_low_same_length_divide_and_conquer(&mut out, &xs, &ys, &mut scratch);
        assert_eq!(&out[..len], out_after);

        verify_mul_low_2(&xs, &ys, out_after);
    };
    // - MAYBE_RANGE_BASECASE && n < MUL_TOOM22THRESHOLD * 36 / (36 - 11)
    // - n1 < MULLO_BASECASE_THRESHOLD
    test(vec![1; 3], series(1, 3), vec![5; 8], &[1, 3, 6]);
    test(
        vec![100, 101, 102],
        vec![102, 101, 100],
        vec![10; 7],
        &[10200, 20402, 30605],
    );
    test(
        vec![u32::MAX; 3],
        vec![u32::MAX; 3],
        vec![10; 6],
        &[1, 0, 0],
    );
    // - n < MUL_TOOM44THRESHOLD * 40 / (40 - 9)
    // - n1 >= MULLO_BASECASE_THRESHOLD
    test(
        vec![
            461989387, 2665399848, 3579300374, 878189998, 4019917942, 1906890030, 3540714906,
            785021435, 2655957692, 3564967487, 2215671253, 2954375312, 3945786785, 2381932623,
            570517012, 897945067, 890538555, 4176726399, 1938024252, 1214448000, 2607894539,
            1325279234, 1564558613, 987829481, 2178327923, 4147856447, 4083057668, 785634261,
            2167973152, 1652571464, 70145184, 217938310, 1833392838, 807859939, 1395394971,
            2852311871, 2904303228, 3198169893, 3331434354, 735263109, 4147668831, 2611779917,
            2853615276, 1375955505, 2451249431, 4226040869, 171021557, 1972314766, 277184426,
            2375785672, 2222399598, 810164152, 1039489350, 2933759445, 3534834167, 1138611909,
            672716981, 2084719089, 538544765, 3894759873, 937996537, 1412775694, 3879993313,
            1085679676, 1967211093, 1504744432, 3666320291, 2237452530, 1826697476, 3221257357,
            580543441, 4245347387, 4208784414, 3482952509, 1951149172, 3708033640, 586662200,
            345242773, 2377487665, 3100161675, 3699715979, 1377295273, 2132506492, 293803895,
            1798986191, 4197257911, 627991308, 2783841978, 3918540249, 29355619, 3412985598,
            1519516456, 3710419939, 721212175, 3932373394, 4152619047, 2587596408, 456847065,
            1492972877, 2878578092, 4224013690, 4183847837, 3441313194, 715397057, 4270121821,
            1405478162, 3277147812, 3938142059, 2533309358, 1504116963, 3653366746, 2109153696,
            2511274585, 90325377, 1647825264, 836765640, 1125067095, 3544759944, 2652860738,
            1079540720, 1986083965, 3960406547, 349138094, 2949875969, 2733651822, 3091427333,
            3570148981, 733070163, 4233388956, 3214072096, 3112563117, 1651509303, 2142845704,
            1051620120, 3770606367, 4159133142, 2506712000, 1161310860, 1621911269, 2878403318,
            206258321, 1812136445, 3881335928, 703342820, 4106909674, 3002521966, 2923723774,
            3098805336, 1503909373, 827876722, 188113568, 342517338, 743560400, 2994196952,
            1909518945, 4098343484, 1410898722, 1188050013, 3739587846, 1187118353, 2097511427,
            4268272529, 3038427753, 3211203785, 4227105159, 1129593228, 2044890920, 2369860975,
            2369399578, 1454306337, 91937919, 798293675, 163666826, 774359002, 1184861045,
            717591688, 1661912106, 2791047481, 2777706888, 3878079014, 1418504920, 2448450319,
            1922646614, 3068210241, 16469458, 4024185119, 1215576054, 311246292, 4059710876,
            4265782141, 4187978245, 4278365409, 4288916077, 1305069248, 1563951794, 2960618962,
            2009022918, 1272389839, 2439394275, 4208883918, 1504352264, 369924536, 13357077,
            1532849248, 485135885, 2924605508, 841648515, 705817258, 954451953, 3740323831,
            2757913511, 3424971384, 2775642081, 1876912734, 630579777, 747682898, 3056441789,
            2407022149, 3613070805, 2027387425, 4277873093, 3802579285, 744104253, 2776789109,
            1370095770, 1305744491, 18166236, 304885422, 2971797302, 3549745966, 741626137,
            795961656, 1741115244, 3220360703, 4121898682, 2553071415, 3851952692, 3272833446,
            1749035103, 3660315457, 815620200, 4112719780, 4169868140, 111675416, 572996149,
            527650323, 3028027759, 3074169491, 3295530784, 1548029727, 410371832, 1384175276,
            1019155445, 940733503, 3362087055, 313802022, 1997544491, 472489619, 1402431484,
            3640929705, 203857340, 399422159, 2714826274, 1480889873, 2185368870, 2143561855,
            837780556, 4244388174, 347736364, 183686097, 896312971, 774613537, 3413697161,
            3233389068, 4019739363, 337295178, 1832529524, 1289908799,
        ],
        vec![
            3152074368, 2609279004, 2289393871, 1484468596, 1208431774, 832849643, 288683182,
            3006999416, 1903458849, 3111117159, 3671553126, 427622591, 3190285531, 3311271958,
            234886006, 1671522985, 529046783, 2869877832, 2642573124, 60209762, 3035295220,
            2511225368, 2206515646, 3850823730, 222908649, 2567682433, 2162714751, 490925601,
            1537433919, 3002058493, 2023135946, 875430531, 4129401376, 1327326767, 752062706,
            4112039026, 3726450563, 3167143759, 3983527603, 842382530, 3153795246, 3598914654,
            3417317498, 3553459715, 1106685605, 2142189479, 2339845222, 2514744298, 1446296496,
            3075359392, 317750583, 1297434773, 806470314, 3548517324, 2896883965, 1547969571,
            2485824310, 805179503, 4202831898, 597659823, 3535591228, 855958045, 1755923867,
            3365965707, 9682282, 3287632414, 1149281829, 964617017, 4003502184, 1850865215,
            987376895, 22121503, 3262730353, 3067466886, 1231496573, 683689717, 2341771143,
            2402732693, 1733447213, 492218933, 1092253813, 3015603177, 1300236300, 2607300207,
            2327157037, 1107197558, 1862484547, 970467880, 1116477123, 1727183986, 1866387311,
            4294876996, 2433423873, 1947645522, 3490401276, 3091887938, 2150744771, 1590400854,
            4235775628, 2260414990, 2867210260, 1289781050, 3796740921, 1443787618, 2784677595,
            2242443647, 2083926475, 492343557, 4143467814, 4259917315, 2370814617, 1512402965,
            1165630944, 1172451831, 448126667, 2504642429, 2943134913, 253722272, 3930555887,
            236043678, 2028066178, 2274670726, 2117908175, 1814596459, 1589398655, 3643981976,
            3708172828, 2628370440, 2994406702, 3768287526, 3697661671, 72250477, 1952682906,
            3890984670, 2335366358, 1486511900, 3932950080, 4154600910, 1641913905, 1181402070,
            1236047930, 3178973441, 3150857560, 3584289380, 609401228, 733328893, 3965865377,
            3698014337, 2769360864, 1034884438, 739582397, 1922193210, 1130663039, 2639094031,
            2919470545, 3614476088, 1565067122, 120324146, 2567666261, 39022936, 4114703092,
            165126137, 2949477884, 2405325064, 2237215558, 2498656017, 1425552926, 1121499119,
            1406111625, 3405344040, 1219643487, 3602348463, 512125432, 1367217197, 1477847901,
            2226582448, 1871611823, 1134590604, 3982040908, 1623284673, 1857753233, 321590484,
            3220580378, 727231217, 983590217, 3953357093, 514378029, 3972342992, 2131575137,
            3154014995, 1075955265, 1966241007, 3201815871, 3097020322, 3758733690, 762231963,
            344183707, 2857398231, 3337254067, 218375481, 882367684, 2204572067, 1038287244,
            3321266584, 2994716041, 1768291972, 3957932586, 3024157312, 621247724, 1627210559,
            3958490863, 2281045736, 3555698779, 3902429680, 426592368, 3133450642, 4010080363,
            1875043293, 3628899254, 2601848204, 663261663, 1664093718, 3328282988, 2238354920,
            4012861950, 920971329, 1582693646, 3875328875, 3096546242, 2216996167, 347539721,
            454900186, 2025219395, 1756065860, 1337774773, 1867666187, 584376262, 2443135798,
            699177444, 2101251123, 1815621294, 1747522047, 4180260619, 2565611399, 3614008442,
            981733561, 339547618, 1433098485, 523523039, 756060971, 432087064, 1056119297,
            3262021204, 2309315188, 36557289, 3784506551, 3467316194, 3626135421, 2707559086,
            895034761, 805184221, 2207233543, 2613952626, 2493110341, 1147976987, 3110836167,
            738569324, 2424079556, 2624128221, 441916235, 2974993618, 1131709928, 1556781361,
            2518935270, 2504025847, 61629344, 3838407260, 1506641084,
        ],
        vec![10; 556],
        &[
            4070586240, 2326530569, 2261395410, 825689729, 2475737777, 2607579621, 2779055768,
            1031674491, 1535775813, 3799159704, 2509339137, 642496935, 1388726819, 253797856,
            2389660485, 1933170407, 612455389, 4107963146, 1906296838, 2898497315, 2050606412,
            3001222322, 1228059054, 2396274750, 2706199930, 3790047803, 1941351862, 1789409896,
            1715227383, 4265202904, 256846097, 2002262811, 1019015022, 2082931322, 892118471,
            1902650557, 1821044055, 628436942, 992944813, 2152035281, 3205031699, 3012731022,
            640270255, 3942556100, 4180900492, 3316509849, 1950826436, 3810257818, 2224553717,
            1511731290, 3648751071, 26431626, 4214594419, 3417633336, 1456941539, 858305585,
            1967412985, 3623211292, 3984085024, 1384481949, 747470989, 3106003382, 1550527069,
            3614045491, 2344422097, 373587226, 2547151647, 994917426, 2913847676, 2378129020,
            555898239, 1108784126, 1609305655, 2815855409, 2517331928, 235971500, 4146164385,
            513982484, 3224855761, 616267314, 3033012116, 944903736, 4143585656, 1155526042,
            1713184683, 1067605397, 4185794190, 3837642784, 2629382351, 937694634, 807052082,
            200210144, 925428999, 2855412451, 351040428, 3193751061, 1327138015, 1236708649,
            762670968, 637002031, 3265514324, 1325649336, 418630090, 2004473621, 395337679,
            1254733858, 2979245588, 2917625161, 4154932643, 333114873, 4020812672, 196942723,
            3805667762, 1295056656, 757090141, 1581564716, 3028221550, 652769076, 2520564087,
            1804643123, 597439055, 2022877640, 2295355063, 2554644875, 2505050304, 1251254106,
            766564249, 736769858, 655024585, 3983536005, 2807999031, 2526649460, 3997807999,
            1027142135, 3147220153, 2611110579, 1353792039, 96665204, 3799354781, 1169870906,
            163280197, 1201147091, 506111450, 560981310, 2487787507, 293569384, 1660966093,
            4172508795, 2079157727, 427559941, 2199181463, 766276398, 3590398380, 921568940,
            1978512327, 2526895800, 155351140, 1734194686, 4286806742, 1665643514, 2910888313,
            1105506458, 2792013812, 84803204, 2779601075, 3530726950, 2109503137, 447716492,
            249291965, 871214282, 1950969439, 831198533, 2725086482, 1111889448, 3879479978,
            2578937795, 2360329247, 849485221, 3118220783, 3215737700, 2629034929, 1803802428,
            579744897, 1521586187, 3794906163, 1682984424, 2501371511, 382339060, 1548025441,
            3993119644, 2642540168, 445700200, 973327482, 672676716, 3810693263, 3980560630,
            633737607, 2979566412, 3035890974, 2663806368, 3077368948, 559094664, 873179537,
            3907578303, 2898488985, 559532336, 1951693750, 3221546487, 519175878, 3817909883,
            4143965836, 2159038430, 2155451492, 984127081, 1552180477, 3998228380, 951987999,
            3271220480, 3991577644, 189427243, 1451593380, 2417528240, 2707909762, 2545325149,
            4092625458, 1678828086, 65074887, 2912853674, 2298196517, 1962123671, 3156663717,
            1809562653, 3422933930, 3543349121, 1539368201, 4132862547, 2117037713, 697944662,
            3761970455, 3210335685, 4061395188, 1399948007, 2797740984, 523361390, 2891000343,
            4043976998, 3948000788, 2806803391, 372268870, 3066648573, 1552197320, 1104893676,
            1977956692, 4206072635, 1777799765, 2593085983, 909587553, 701089400, 3025031477,
            3150914776, 3383493968, 3297968979, 2127114185, 347641566, 1133943045, 1397026960,
            1102135376, 1790048118, 1531899702, 38573247, 86358464, 807071035, 1322974764,
            3918796486, 2658009039, 1875612194, 2278918363, 4156398458,
        ],
    );
    // - n < MUL_TOOM8H_THRESHOLD * 10 / 9
    test(
        vec![
            1131150821, 2762649368, 1291624341, 1701704353, 4014105561, 3682713698, 514896869,
            2382721993, 4199508292, 116228850, 1913167611, 2543927978, 3462810307, 2182535806,
            3234802163, 4243621064, 4189108519, 2844896515, 265654095, 2925399216, 1942537721,
            3688510290, 1467767823, 1331956526, 1810538674, 3416955279, 2622944711, 3883742491,
            3716395614, 2226266870, 4231113757, 1703472377, 799710037, 3188891105, 2002885312,
            863976785, 2197487335, 3973964671, 982134718, 3039352665, 4058841706, 3126556723,
            1839202656, 3512286394, 1526861314, 2857382087, 1509071714, 3983699181, 4164670109,
            3541034768, 3414363792, 3719311258, 4228436631, 418239225, 2810981500, 2839232135,
            75036670, 349836787, 995927099, 1154560390, 1157124967, 2338833107, 3519575136,
            3676349342, 1999912677, 1473285447, 3384834599, 3729173386, 837302670, 1824368274,
            4080054412, 3463347047, 2292431457, 4047304953, 3186147982, 797486141, 2414391350,
            621753596, 2992059534, 1538383233, 664053981, 3061928211, 3215272269, 266835070,
            2414626927, 159250143, 3019791116, 3731135237, 2761751093, 3586027201, 2089419276,
            374111952, 49970046, 2897511132, 849093074, 1002730932, 3171254030, 477351679,
            349956886, 1571756720, 2974023410, 2229070678, 1445469779, 3125204262, 2251629977,
            3617654578, 3758587650, 3379646707, 3221020164, 3946481152, 746980266, 445806733,
            3092441206, 2414172109, 45651552, 3218735738, 1981382211, 3339893115, 577943370,
            3262981881, 3145699413, 3050111476, 2776211838, 888798694, 1098018756, 2384093069,
            137109573, 3849877919, 816321891, 733291770, 1688860823, 3776150706, 259328987,
            1793361953, 1360633863, 2822514209, 1611568653, 3671864876, 2995705621, 1703631428,
            2545904596, 3558099294, 339073366, 1914245190, 2248240708, 2918919078, 2367951631,
            3244616755, 1246248837, 2724721481, 1977303106, 853273015, 609224845, 2572706277,
            4120826908, 3132172741, 2679460826, 1731116641, 1420934603, 4095172779, 2134926401,
            998210691, 3678534171, 2288544872, 2333609559, 3187493478, 497033878, 13547027,
            1366761551, 3620514533, 1846847912, 3066898750, 4029310316, 1785305517, 2975988788,
            84028222, 1604571422, 159814579, 3302598788, 1860412674, 3924970651, 3084289036,
            299916336, 2801290202, 1879794819, 3343450230, 2673079282, 3615958442, 3162274325,
            3102296518, 435434416, 2915209493, 1817171393, 1159766844, 574519135, 3033326838,
            2027440653, 1457685030, 2574873438, 877258212, 1477651381, 258103620, 4236228819,
            1480006308, 2237899379, 4229629566, 1151532728, 1869635395, 3423452052, 3699388043,
            91696809, 1622972763, 822248470, 1515292231, 3842486533, 3927210641, 3225680067,
            4254069260, 3886678620, 3984473786, 2759203157, 2517007583, 145365022, 1533952248,
            3917863954, 1567125726, 3108296518, 1454586200, 2011082282, 2132338201, 3182831171,
            1413497090, 1856759670, 1984669926, 4212692874, 1110589546, 1440032735, 3646801244,
            3610860056, 3043101171, 3857863470, 708594714, 3801293829, 84401414, 4009945641,
            718064348, 1782564232, 2884677460, 3817956052, 2164661851, 1032113674, 2265026833,
            3990818483, 1979791851, 1179815111, 1179610990, 3801644913, 2951826902, 1933447922,
            1907000683, 2045622439, 3615799713, 4178470833, 488268281, 1409188637, 1338007608,
            3944782180, 3885373568, 3104460402, 3927888479, 471271819, 3870427647, 4196319798,
            3583191843, 3310030186, 860772115, 2433189612, 830226736, 3738387750, 1448499320,
            526587241, 3892532590, 3439491908, 1039036501, 2388493771, 264726903, 3966258028,
            2505867608, 407881715, 1631490147, 2823006404, 3497117958, 646382286, 3107394453,
            3277949521, 439910091, 309065703, 1950804341, 1808416502, 170990188, 1335198213,
            689918674, 1582196175, 2640621496, 3498813308, 2761060243, 1678685681, 454638300,
            4004659669, 3681198397, 3976254726, 1940449540, 212889053, 1912817590, 1301852137,
            2300183575, 2626591779, 3662656365, 2241640523, 2073990699, 2843210521, 3907897499,
            2127053203, 3464740089, 2517063724, 849818543, 2198029686, 1607835696, 2158656964,
            2474427278, 3105729582, 2593228154, 3652599476, 2493295322, 4214474951, 2092751192,
            3508872416, 4135027724, 1087631006, 3869783862, 4127959478, 3950654826, 1413528055,
            1583176859, 3118004380, 3469658789, 166660010, 3732069246, 1165982209, 3249209142,
            3030024737, 4209637631, 1574902930, 612943597, 3575711404, 2421873039, 3084377476,
            826443943, 3566117097, 1367287788, 884174375, 160667863, 839942561, 124546248,
            3443852897, 1935836700, 3578696710, 3442653707, 3106215554, 956073431, 352475044,
            3325573670, 4056377273, 2444639044, 4059211486, 3187262860, 1232076586, 712845366,
            2490553931, 750933277, 1200249747, 4114847099, 3132331434, 426349171, 3418210348,
            1864124410, 3635934842, 1778322792, 1268045228, 1155805234, 3891131459, 78081971,
            2759073032, 2990099706, 752418656, 1424173464, 2440522876, 1954352637, 3476425702,
            1980817220, 3283383787, 3606231400, 510762389, 4244763535, 2869358935, 1727272135,
            3522246353, 1733965890, 274429005, 1074555950, 1116810589, 2059372124, 1433737439,
            686360791, 3524954413, 790420924, 253262937, 2490041017, 2988382494, 2680296403,
            740237358, 638968890, 3561449712, 1270014183, 2732717194, 2652066843, 1127846686,
            1905112070, 7000223, 2815346469, 3485672034, 4150256142, 1201712608, 782060943,
            3850587668, 891222391, 3919270279, 2284997530, 3355185596, 853556214, 2382672975,
            4082635614, 1975597308, 775008313, 1221881305, 318112932, 1558885629, 1659368995,
            2737245040, 3182289543, 2865953307, 1913563745, 3407843590, 2774667399, 2006733037,
            214221440, 2867329721, 1460609363, 1420414655, 2442726895, 2077873651, 2078741260,
            3351565491, 2650010589, 3624853117, 2808835283, 3576673605, 3312161027, 1089529182,
            3411999136, 3214717560, 2569067404, 1395383167, 2628670095, 2534091665, 3416849349,
            602644653, 3868242106, 780173614, 2918196647, 1069725684, 3173740663, 3441776760,
            1918895783, 2591986841, 2047011602, 4037432, 633731206, 2672531137, 3708159409,
            2150429215, 954500477, 3990097385, 1947492698, 981465130, 314252035, 358539692,
            683336340, 2361937249, 2060146559, 232682027, 87274822, 3435325468, 4227641462,
            882380594, 231040832, 3746812704, 502289657, 830742778, 3389856631, 2652175412,
            3688357856, 3281892315, 1707485812, 227398450, 531784725, 1476141529, 2868884010,
            1656814989, 2816953357, 1091559808, 1917641087, 1677755142, 548205597, 175354774,
            3985092082, 3309818895, 464474156, 4204834984, 3761500115, 2438523503, 1732610689,
            1486464139, 3967200098, 3929926820, 2339786049, 3907108412, 2199568695, 1134103910,
            2216911633, 2182031298, 2579739862, 570100824, 3289356324, 3458481331, 611877571,
            1507859299, 2274314160, 1548661374, 903468480, 264167483, 2119844238, 1038239445,
            2988435265, 2038849220, 2021865993, 666366893, 1683542644, 663579226, 2516381838,
            1452293108, 2860570530, 4048880023, 4066088025, 3363037289, 2542443950, 2325290119,
            4126749311, 3549516466, 2259054696, 1124113431, 2596211298, 926851034, 4100110238,
            619577504, 465050718, 921970188, 3443237807, 3185995153, 2512437958, 3138724608,
            3421198187, 1215660513, 2802265359, 3639819408, 698058093, 4005388493, 701032232,
            1900034464, 1258990473, 1384847467, 47742046, 2533738470, 4033217779, 193441436,
            1167926881, 563345579, 1064730454, 1028831057, 2118889296, 1922177050, 3759770599,
            762133197, 3305899630, 1774669125, 2184377046, 3697369449, 4032191246, 3613695996,
            2714184171, 672093641, 2806515914, 754799026, 2811057556, 3677282514, 1644648375,
            519012895, 2947700042, 1068870929, 1749850446, 2745609943, 3915641236, 1473746566,
            245175508, 464459969, 2300554864, 452081255, 3542824159, 3984931523, 4038093048,
            811698203, 3990065017, 1843168045, 102873045, 1872945131, 1381891297, 2689743436,
            156330831, 3030944325, 2925864612, 868161142, 1955093372, 2271793313, 1293536600,
            3877989118, 4085300196, 20015595, 1524783113, 3220067646, 1128665882, 1605056207,
            3769938324, 3278249586, 865516226, 2018942727, 4285193298, 4136394815, 18140432,
            1298007913, 619558910, 1468345138, 3640156579, 3844459024, 4251273955, 1279325353,
            1490675071, 3812253034, 1588374415, 1408352173, 1856222658, 3479408648, 2553632750,
            539602295, 2953721539, 3550404522, 2987339861, 3573803000, 2985536505, 3408426662,
            394679673, 2338150855, 233169574, 2666618942, 885970658, 1261112849, 3026593914,
            1941961461, 3178744470, 3418317636, 2483060546, 247068105, 3869710296, 2601474183,
            1861230264, 466844147, 365421277, 1647470055, 2545886518, 3344491344, 2743895143,
            2612409420, 3015491275, 2263288257, 3788192586, 3621947232, 3632971491, 2590893591,
            3078266930, 837964625, 1062932151, 2387310193, 4263324106, 3590937645, 2690575361,
            3471028939, 3052123108, 1287391629, 2780458428, 2899985132, 3768032123, 593582914,
            1049982970, 3417846725, 2745208959, 2638635678, 2782226812, 723243488, 2883204599,
            1861655363, 2193262251, 460750285, 408576776, 3151751571, 2348465332, 1090324024,
            2080133910, 1884601095, 1792700738, 1586206119, 588433151, 1694897587, 3243922990,
            3147195067, 704523940, 715280464, 2859196369, 2818686408, 3845709444, 388946443,
            3822371361, 1083710054, 3973525343, 890875638, 336962766, 879786542, 3372880597,
            4017818517, 4022694260, 1246618400, 3272410744, 2932065660, 3037109199, 1994060649,
            934925733, 2550662182, 3369254727, 3300198891, 1222413593, 364996956, 2915355551,
            2528303182, 1340653763, 6095745, 430607376, 2462322073, 82876788, 1168959908,
            1417636305, 1265099813, 274745957, 167508621, 3915190887, 4169873122, 2699401240,
            349817411, 1083358516, 378109903, 1137415528, 2610418640, 133616096, 1079889395,
            1596538242, 1340572700, 3806709899, 2995810782, 196324185, 2670063802, 2058666273,
            2243102740, 2109579437, 3319920398, 388799298, 1644474748, 3835051237, 2938119592,
            857157002, 3904318910, 3191603530, 4005173949, 2864727253, 1049195541, 1431710706,
            2394529501, 314388331, 3586260934, 1413117985, 1378439215, 90290944, 1187129087,
            2806417408, 2252578657, 731737960, 3976349288, 2835651723, 3336636332, 1381984503,
            1669124519, 3410912262, 3447243564, 222368697, 1603262029, 195964253, 1003881737,
            3008564716, 3332124267, 3723209179, 48076559, 3962702541, 3175563645, 3613059603,
            170958653, 1413948782, 2426947383, 1103046276, 3139837644, 502007526, 877737020,
            407264876, 765859135, 1474201609, 2991446344, 3468949411, 3551131726, 560361863,
            3036580569, 3430955843, 1011987611, 3299206602, 1953068741, 1760110431, 1246003603,
            1099977647, 39794154, 389004595, 3989513261, 2907906071, 1622738740, 458000748,
            1389814785, 699501716, 2230559, 2553609548, 1451109816, 3441722227, 3416954130,
            2885301526, 1740199906, 1803806248, 1403790177, 3516185322, 457833177, 1470646316,
            3450277077, 2082948067, 2481239364, 722376046, 2758592798, 2482526812, 4010504381,
            3378951917, 1539096335, 1599687582, 2811560884, 2435502051, 2900235322, 3627166120,
            32186692, 701703510, 2447124866, 1509277763, 2326977874, 1288896304, 230630067,
            4134237779, 2960340409, 2917586567, 1164733926, 24754896, 2143494083, 3622160753,
            1940106076, 3135397968, 3020065317, 1748012748, 1014590694, 162297287, 1327741722,
            746744365, 3185502087, 158350155,
        ],
        vec![
            603622954, 2186113344, 2211632492, 2411520388, 3589635543, 4086364596, 3581723392,
            2612098311, 2096044537, 1640167886, 3342490559, 2125968601, 1990634435, 312775010,
            1802349863, 1309411103, 3200864131, 1358335036, 1912893833, 1596337323, 2247665998,
            2443116949, 2420155681, 841076086, 377456615, 1237592486, 3216360690, 1624285201,
            3654260727, 2790733511, 204168525, 4052634612, 2854194678, 3030697959, 948115195,
            3826905149, 1976102477, 1784393101, 2995452496, 3261601101, 2842141116, 191128211,
            1816171240, 2015310936, 2049251282, 613801401, 3835986911, 2470139377, 1942210663,
            3899898568, 2985108866, 466048082, 1427695846, 860088895, 1905116312, 17072967,
            3747648444, 4050068534, 1235378754, 3463008343, 2901169501, 3732797830, 3827542306,
            1442668438, 2553056440, 3128009710, 2361840470, 2945391897, 2844092880, 1669579593,
            285906550, 271337390, 3550080313, 1550684000, 761190775, 1986821670, 2760227175,
            3755739359, 3195863504, 1506193392, 102572887, 3429355396, 2738466594, 2588290861,
            921241140, 4000693169, 4200573367, 494632512, 3030208655, 4097675912, 3273014929,
            1507497783, 1426537214, 1292494848, 737012202, 840807439, 4173254089, 2614555500,
            2112014072, 524226642, 4134209698, 3019212395, 1098411347, 1127762058, 2520698217,
            3618515874, 2710325152, 608562040, 3514989488, 4158363396, 306180876, 1211447956,
            507700449, 911736750, 920771768, 3055911518, 2660700314, 3162062346, 460147980,
            3893374896, 3335254211, 1730373325, 1769502379, 328248258, 3605468276, 839999978,
            2617035765, 3256832677, 4166868036, 3034242873, 866786060, 1439047189, 1075047697,
            3791824427, 2428657149, 1942106734, 3406726485, 1061929031, 1911794305, 2055938947,
            4130327586, 3752449664, 1741794117, 2088328119, 625641627, 315760669, 2840762267,
            327041843, 2880804433, 3266656983, 2181853630, 349055847, 1614300853, 3026780591,
            3723351880, 4108567046, 2344446900, 2195753474, 1660780233, 2121691315, 102176310,
            2695465705, 3089322092, 2775472883, 2630273841, 2862312016, 2071059637, 1584923974,
            609930669, 631828585, 1504369275, 892077659, 3793274532, 111247066, 576379360,
            3257765683, 2389261674, 2210132561, 623967505, 3475583657, 4078514517, 2696227314,
            423351810, 219536780, 1292034857, 3134390109, 2644943026, 2175174277, 854381939,
            1615329948, 1003209984, 1497561541, 1581742618, 3453870211, 1697610716, 1386823953,
            1754275998, 1049523617, 319480181, 52790852, 1843711621, 1677461984, 2268587847,
            1467575383, 809390384, 3208538160, 1710678484, 3852975493, 3491099676, 2208895838,
            67474001, 1909240027, 552833552, 1068516200, 797958994, 1336265228, 52799811,
            3497387824, 2615050498, 2306496091, 1509488357, 3175308944, 1400875928, 2613915127,
            4014015840, 3770610211, 4050294544, 1133059937, 1931794550, 609394840, 730823155,
            2274984932, 856801557, 2940954445, 1507833826, 3361523681, 4114500875, 4016167065,
            4145941303, 3897868338, 2892060643, 2413714784, 254604231, 966264223, 3162224073,
            4257953665, 1775093780, 4059868817, 229216133, 3095801819, 2780776724, 4197103660,
            3945827647, 3590005880, 3385750965, 588116955, 933606134, 2954149398, 3136897257,
            3742737011, 1904500440, 201319529, 1627670998, 1436294506, 2391068109, 2754913831,
            617553192, 1237348537, 155331642, 666659293, 2505670832, 589051162, 1617401511,
            778060751, 2180910952, 4141824369, 600328318, 4045490772, 4118525083, 916092943,
            3026682184, 2750156732, 1451420324, 1940219622, 2451125563, 1894319083, 712954796,
            1241142933, 1547315725, 2191677486, 3213688112, 3550409294, 1708221443, 295690779,
            4139342911, 4044286844, 2827851250, 3745891811, 1142267160, 1568307568, 4187843675,
            2744250115, 3543324184, 3231805406, 844623162, 552365416, 2085517327, 2950237187,
            3595863774, 1726025744, 3066980951, 2635434821, 3786966597, 3778968631, 2367918512,
            1544339264, 3675143945, 2263950208, 256073448, 570000958, 2196435922, 3645119110,
            1544421656, 3665474053, 2710076922, 3456896575, 1661288153, 798098177, 3205290044,
            3267217028, 2164126201, 991695109, 2360577181, 736096951, 580335383, 440415808,
            2970181697, 1174793452, 2790186381, 154139411, 682753359, 1362922244, 3445269063,
            3850397629, 3630608237, 999211182, 4270325568, 2222213569, 1671798056, 898355980,
            1426188949, 3343654679, 2919951295, 3693477162, 3895401157, 3560721913, 1888596458,
            181288724, 3829829389, 1579972422, 3144117276, 199801876, 4054633683, 525925388,
            3648577609, 286807073, 4061577217, 3752567147, 1201522573, 3610440980, 3336089737,
            4143647920, 1346119142, 356944187, 3589071297, 331350548, 443179702, 4287676723,
            3902224692, 670781524, 1151314689, 2224154332, 2643145761, 2418685337, 2754610868,
            1367435799, 989379597, 3280309063, 664367389, 3695457682, 2932182363, 1637715984,
            3078239282, 3890423242, 694186670, 1022735416, 3766855053, 96343600, 2950575721,
            3694574064, 2063490620, 628896960, 3009419511, 650278649, 217113522, 4054867950,
            4085775949, 2253647601, 1323118326, 2776229090, 387312318, 3830031367, 2855934165,
            1727237588, 3168211450, 985024091, 4065976202, 4100263284, 920460504, 1019722484,
            736802555, 1091160058, 3986567293, 3909716414, 2117717711, 1378801332, 3582605439,
            3974837927, 1975607076, 341568791, 772919682, 1404531926, 2319942259, 2541979115,
            1537431363, 3469010753, 2351635824, 2574168815, 471490424, 429652049, 1888748017,
            256119513, 2622167989, 3890927619, 734423772, 2108123100, 2720692858, 3149485041,
            3738523581, 2699573275, 2549388345, 2521302893, 2530474986, 3520669288, 417822002,
            3240327386, 1661401307, 882983082, 1924087083, 4263722671, 154785737, 2892272366,
            2096794030, 3065733707, 1274676481, 3173744694, 3170057336, 2560528531, 1992114189,
            763608132, 886242142, 2381935549, 2696870385, 1183400652, 632435857, 1114549236,
            792694237, 1050118375, 3043570987, 408243821, 3246069230, 3595014201, 4242308859,
            2732256078, 417888811, 157189594, 1578645677, 979394949, 2715618148, 2084946492,
            2779545872, 2746594737, 3600471957, 3690418809, 80909094, 479165038, 1299019782,
            4248583974, 2122215085, 4100895072, 1510073775, 4108853940, 21876970, 3804082593,
            1194473031, 1013294843, 797533856, 3555511188, 712992910, 3147433210, 230132027,
            1173487980, 4269226972, 3090952473, 1190923232, 1679528279, 769279893, 1422455341,
            1485744354, 391718383, 376237997, 2772702841, 3412931239, 3901610169, 2108304638,
            2830169940, 3849960095, 3043501233, 3973846143, 1627749584, 2509012508, 2993028673,
            3443779088, 2629553481, 3345770078, 417661904, 299926134, 4221411771, 4277090614,
            1911866219, 1551326585, 93131922, 2142950833, 998841580, 2131273819, 264704024,
            72761473, 311706753, 2773201027, 1819604430, 1325781384, 1828296785, 475315722,
            2752215033, 140320306, 1043096704, 855365740, 2476726857, 2581734598, 3861106067,
            1436208963, 3415450549, 3570329317, 2687213974, 2613842059, 516462296, 3037378709,
            3263570551, 1681420526, 3415302057, 3511450549, 3273504333, 494102854, 304288318,
            3697281980, 1996479758, 1635215124, 47632440, 22355542, 906188237, 4149186094,
            3151082744, 2531819048, 1644757840, 3101983578, 2011269244, 3866138172, 2127822384,
            1903103074, 3932219954, 1821169710, 169278046, 653152755, 1929628869, 1986843845,
            1656941273, 1820043656, 2237572096, 3650186585, 3171963679, 4233697440, 3755777109,
            3448968195, 133782836, 4267699280, 2851793115, 617641793, 3256436523, 1407468314,
            4096930119, 1868261036, 2551660459, 17209088, 1655418735, 499829078, 3115810383,
            2942436423, 1117668901, 4054638152, 3401715738, 386251371, 1788551278, 541357881,
            4092916714, 2788358548, 400852308, 2632328808, 3366533041, 901146853, 3250917102,
            1993332902, 1740749354, 3817981431, 2227569979, 4241229228, 65274376, 1935767139,
            3249336235, 3922555776, 921336913, 2814396307, 2667550257, 2055331186, 30339902,
            845791404, 3302805554, 2378743215, 3928499493, 2035920961, 4033017204, 3562284680,
            2948673159, 1367405928, 2477125781, 724878315, 618610843, 3464131396, 4106290593,
            2761582218, 1911670733, 2255224976, 679995901, 1694101202, 1457350452, 2415077238,
            3832605769, 1548512392, 3910956093, 2960448392, 235679115, 1336039703, 4056841792,
            2704719451, 1221241812, 1530676776, 758875004, 3989933820, 2315326962, 1755061483,
            984103973, 3552706434, 4146283304, 1900596987, 3224925366, 4289171038, 980557750,
            2062175350, 3065172916, 108844606, 293646367, 3415841496, 152825526, 2072381275,
            4285030108, 2847377365, 1313245709, 1976515833, 484145483, 2307919762, 1902482190,
            1346944178, 353282945, 4113734314, 1863262682, 1245717043, 2241864885, 2280528335,
            1019309397, 3289776976, 2376807409, 2952610417, 103415167, 3638228557, 3235194883,
            3241161224, 2125883305, 1232190514, 3549022975, 149807933, 2961278754, 2383239375,
            127409706, 223990857, 1600263562, 3325399391, 2741861086, 244824352, 85948473,
            3124738586, 3484658404, 2949005416, 3110472205, 1541858124, 3817776205, 69164118,
            2954933199, 2925668013, 162217966, 547673095, 4181924147, 3823145053, 80676848,
            868737602, 2205414045, 2279798936, 102486197, 2587491480, 2241109714, 3451004219,
            1101855332, 1254174284, 4162654397, 2075550533, 3279994730, 511501950, 1774750940,
            2330903266, 2010908807, 1405100744, 2399290465, 2186798761, 859965651, 680542282,
            2464087287, 1717488673, 1106965230, 132594854, 1192599975, 2168482389, 2337484280,
            1431281627, 2161439863, 731837641, 753400083, 718352079, 2920193564, 4014452495,
            2840575861, 850092154, 4013385863, 2889779981, 1887731992, 877392739, 3501145229,
            1077314268, 1838346422, 1325751328, 2289785549, 1040644978, 3430927961, 2641937927,
            2285310825, 2723705356, 3969476694, 193167645, 2688798767, 1812500605, 693030278,
            3173921056, 2188596291, 2180502082, 2208231604, 1512410467, 2765076968, 3989339249,
            2744873321, 422780839, 2753112367, 861241423, 905851909, 1219406342, 3083085811,
            2111384879, 2487677895, 362839625, 2300647559, 2252314956, 38471328, 3558718806,
            1713409176, 708493886, 2645700082, 2196532746, 1397262287, 612919750, 1411685900,
            1134021085, 3675861851, 2168510458, 173154823, 2489748386, 2499456938, 2828871069,
            3373056478, 2660114090, 2270829594, 3459032228, 1357929287, 1162183927, 1097818577,
            1542913661, 1981968930, 2944755083, 579671294, 147480949, 1988196545, 777407838,
            2354930885, 2534601251, 2766511651, 68413970, 3533296816, 2575956638, 1355214150,
            2719147503, 850624539, 3284264812, 3928845488, 594971150, 549980540, 1796991397,
            776876250, 1460934050, 2463458608, 1640602524, 2160282188, 381030894, 2276755467,
            3897004968, 1525634949, 2084303318, 4038502302, 650730684, 3893039590, 2120711163,
            1075785968, 642329759, 4286493776, 298483904, 576520111, 670640982, 501843005,
            2317149359, 1723210307, 2714699821, 2647759513, 3372861558, 3302356979, 1135050697,
            1794914040, 2501115240, 1473307986, 4055431991, 386019549, 1642338927, 817175303,
            63704969, 1918346370, 1060455488, 3296205488, 2611534290, 319202585, 3589598600,
            1061424333, 256294593, 2086764847, 1889683341, 324402820, 4230133797, 971777609,
            2913835299, 567526498, 3967034861, 2199121094, 2595932876, 3882641499, 3024547071,
            2251608750, 1562313747, 3646762714,
        ],
        vec![10; 1840],
        &[
            925805970, 771512525, 1321505884, 4093572597, 758716614, 1377765832, 1278203380,
            1001888643, 1355158095, 201085215, 4149044771, 2521855414, 4152533261, 3002132831,
            4155360674, 1795934109, 2247306354, 2834452223, 3656644513, 2792060749, 3230919138,
            483669446, 3693350164, 2766501057, 329182922, 2273808186, 1047344319, 3115798204,
            3566682772, 2408293239, 2794877731, 698720575, 327005792, 1558338384, 1008199867,
            2492809980, 2810143023, 1784882162, 524641759, 928770560, 1130801733, 4222233720,
            195758472, 1891783101, 413324813, 3563105984, 4225585084, 205810396, 2545279346,
            3861389278, 2373143813, 4024119708, 780120208, 3432178227, 1555529931, 2554973234,
            2502444226, 3344683717, 1641184112, 98587931, 637912572, 1021943377, 4037505911,
            614639139, 4194497843, 3049554202, 637648248, 2123348064, 2634366226, 3207837322,
            3066784253, 2994933327, 1976651548, 2366296173, 1641868749, 82351438, 3355135280,
            1553860281, 1605364581, 4075743977, 2941712013, 3386363575, 161305709, 3498025167,
            3591439792, 3723487074, 1286356811, 2869673859, 3058493670, 1421607538, 550853035,
            912260189, 555605432, 2564864712, 2716459709, 3225217761, 1778511468, 3587420558,
            1207869481, 2862990086, 4126739672, 3193798621, 3165324498, 76671852, 3202055203,
            3698858085, 2586219907, 2247010126, 3962725977, 2619019864, 2122584571, 2759072926,
            2355594856, 2680875986, 3809288380, 3113157553, 2998504932, 1438335802, 4042229352,
            3236956253, 415640042, 2304719113, 476088551, 1007695371, 2386089176, 2181311976,
            736590680, 89015838, 391491629, 638291307, 2877449105, 2962906061, 3979685552,
            3627063001, 492677310, 2231326821, 743398531, 1133825751, 1719921083, 1840883029,
            1166898759, 1601037521, 2409186612, 1623857150, 2793429784, 1436913897, 4140973832,
            582129472, 4207088238, 3200623818, 1475586379, 3384915939, 743709146, 2191495369,
            3563177671, 728383298, 3754965841, 3795767466, 4009441303, 406982577, 3135143383,
            1456106797, 4160967347, 2816553701, 3256668188, 1014590891, 728446496, 2331482373,
            2184086986, 2476758117, 815465765, 1957139615, 93446988, 3589331411, 3457536764,
            1745564725, 244069742, 3319690805, 819795556, 1486648551, 402515678, 1904027145,
            2183046265, 1437045043, 3646826056, 3694590855, 3691191903, 912588953, 2869059243,
            915090531, 1308341773, 1914740545, 1372995357, 1821589353, 1750436980, 3326691416,
            1030623833, 1495253922, 34912358, 1499274973, 998994535, 2935614243, 1565643646,
            3474712008, 2524137404, 3968547971, 732339375, 3134376500, 3712532227, 1085750881,
            2607575268, 1414329062, 583507621, 2489115057, 1425727804, 624607973, 3077024829,
            2685992057, 3682897129, 4071136447, 1727489833, 21203108, 3392758652, 509383349,
            3119006823, 3045101600, 4129122439, 976347783, 750957340, 3621119842, 119059944,
            3926834627, 3278710060, 3524620828, 2053225005, 1628028369, 1741363163, 4055830757,
            877491867, 1503296989, 3349399277, 348921314, 2220238102, 2327127101, 371334052,
            3730431190, 4254998900, 3276010967, 884857325, 2799001549, 551761767, 3358096790,
            2020737989, 2913273611, 795353920, 995285043, 1255353878, 1081464851, 1485661898,
            248227391, 2646461730, 288845121, 990971374, 4151523546, 850493727, 427991127,
            2225569843, 1598181146, 924478291, 1006667974, 1424598842, 2544076586, 3839052630,
            2039302967, 1751844817, 4119169517, 1378503947, 2112648581, 486800876, 1589075140,
            1774624325, 3380949714, 942159303, 1934386105, 1853027873, 1625283567, 1798870407,
            945414724, 1916704482, 1076254981, 3546161383, 2299585539, 3247652613, 731481822,
            1011891129, 2768863308, 1813803045, 3860051875, 1174964707, 1324160940, 2574847612,
            3513978083, 2554787041, 68699746, 1212730198, 996237523, 2736940550, 508920579,
            174371335, 1886271926, 3087461373, 2201998413, 385737266, 2695461851, 3309543132,
            1687035998, 692717227, 569529002, 642730495, 1998450569, 2877673596, 782260981,
            714386348, 3796153446, 3810357608, 1220822655, 37559611, 3062930083, 1952046909,
            2938302747, 1937200848, 139230150, 3652241605, 3385303728, 3427356966, 2415007129,
            2062827063, 3399013574, 1412210998, 3780689301, 3309010376, 2776522750, 788159042,
            2338674637, 3597251270, 1111645430, 2344936207, 987533024, 312063196, 2164245832,
            2981634349, 4146252745, 509985595, 473738885, 4080179792, 767682002, 742937862,
            3390341429, 1201520568, 2705803790, 1096534023, 396035895, 3045556830, 262555759,
            2223158076, 2460071430, 537129776, 2525318176, 2465664490, 1718928604, 1677204238,
            3488426978, 3891524565, 2842016557, 2193145485, 978116097, 1615400737, 2757085537,
            2962918904, 2163577224, 4068394581, 3963381668, 3027498111, 413627370, 3985257787,
            1911780465, 1431948560, 899666734, 3073534561, 3931614071, 59372171, 123789420,
            1311815775, 1600081512, 3500852529, 482022179, 1349872136, 2023099554, 1623971264,
            2385783366, 335038849, 4237603646, 1512003698, 1626113404, 3863467432, 503775132,
            185919438, 3413070813, 769549664, 277742984, 1457016660, 3916310980, 3679543908,
            428500520, 291415144, 382143301, 3479675016, 1558773305, 3828015203, 740311290,
            1658202239, 1352230160, 2929801762, 1144182359, 4260518531, 3904249580, 38453053,
            519928994, 611178254, 1815173805, 2687791381, 3576348672, 2210369382, 3101623249,
            98144578, 4159402306, 2478115488, 3668828837, 1846949400, 4015333955, 1478301555,
            2864273669, 2676396505, 2824169479, 2008756433, 871452488, 197076407, 493093639,
            3590505476, 3419603591, 1983646327, 4172389974, 1331126951, 2749313151, 3914363144,
            838421991, 3713866955, 66943799, 3022889908, 905930887, 3450837393, 3122388323,
            3601001575, 1461966185, 1217535252, 862426054, 3891322611, 4057584483, 4137459924,
            3050986698, 3775562735, 2206917997, 1326425299, 2177316293, 665366502, 1789591450,
            3869393426, 4111791353, 2042667844, 3888104816, 477695341, 1400818483, 391421351,
            2511869577, 1543099376, 522205537, 3741717060, 3983781829, 1450661430, 1952049614,
            4059193972, 69169697, 3733732140, 2178094107, 3638192198, 2483181869, 4237963289,
            2635888235, 625414314, 2309845963, 3133618224, 1030061929, 2628713359, 147776022,
            3665551893, 2606716720, 450703348, 1433078851, 665870068, 4274367632, 2346780289,
            34636636, 3570617418, 2610375041, 1570695957, 3235022132, 650696914, 1579637045,
            3924586750, 4122461112, 3973884299, 3203913749, 1074902525, 2038914363, 1313155224,
            280192559, 2654627254, 3904119153, 4289367389, 3931774924, 4088574946, 2110508780,
            1965188425, 3870415513, 3028849533, 2643890058, 3118914716, 4132754639, 2063814685,
            2477010069, 742144369, 3675862178, 1638220213, 1788413389, 3612081483, 2827592335,
            4269582882, 3849921609, 1214642500, 3132507662, 460322913, 3555344490, 2582789072,
            2773409763, 3731720324, 725828860, 4142291526, 1289918833, 2376436341, 2080195956,
            3652068261, 2052975451, 1056381829, 1504404622, 3530088199, 2543774820, 3588528773,
            1181550632, 4134199347, 3106435739, 2666774217, 3422590531, 3887880807, 4070997105,
            1394848137, 701703628, 3061521160, 2887272490, 2229516066, 1310837786, 3154190631,
            711892595, 2435668654, 4100746898, 366709793, 2800825180, 3469579669, 1872527030,
            558351001, 1574511606, 3461209655, 1498408413, 1726832352, 1807209931, 1051547963,
            2514355744, 3694086296, 3693097900, 169040251, 1625096327, 247216005, 1690358091,
            1853931622, 2241580553, 2665852926, 378855913, 2849826467, 2882598381, 1966643823,
            3863600372, 570512458, 89553660, 1580933547, 1670795700, 1701383225, 239477568,
            663996049, 1663598319, 1774418754, 2745157344, 3324324155, 3068121254, 2019212774,
            3363185868, 1017744423, 1422898847, 3076406441, 3863671242, 2045305657, 2537743046,
            3610837965, 956029186, 1782391421, 1492413923, 3211086299, 165767576, 2846934457,
            3007723858, 1141836320, 728275693, 376616587, 2573940874, 2162417375, 2260336910,
            503501143, 165113603, 3530206110, 2914422259, 1430643811, 769845616, 4213816224,
            3629319288, 4239130817, 2081610750, 625758813, 3255558354, 3479654242, 2472621110,
            3007522636, 2750717810, 3286123194, 731654699, 2774722656, 3054704647, 3343723513,
            3353349653, 3916558188, 4002272233, 2120337200, 413996446, 2630363679, 2841712842,
            2347010620, 1903664219, 2118706367, 4121870331, 4262431689, 3571095308, 4041525390,
            3390066445, 927914151, 2343554013, 1134637778, 3558586975, 18737389, 2499628916,
            1647908776, 3587615205, 1651636386, 3701620970, 3344465418, 406209565, 1774097768,
            638612711, 1894889044, 975120431, 1901267275, 3825095883, 3013288190, 1213152309,
            4081208841, 2283263447, 3257861369, 566807683, 766774226, 3630232255, 2719893472,
            2539687707, 3216385587, 2660398423, 654074066, 1010432189, 3431364371, 4163956052,
            3832935048, 156489928, 4188655690, 1306690249, 98975214, 2421098560, 4163230789,
            4156164818, 4443391, 617931997, 4154608614, 100420976, 2222597381, 2417575506,
            2990762034, 204752919, 3415006628, 3522988641, 835715926, 156827736, 2308716058,
            1572542144, 184997832, 157493958, 940407358, 3759427593, 2158763575, 1564687549,
            3909269978, 1283986657, 2367598132, 1568363717, 1710227345, 2793907548, 2427230705,
            2893825635, 3768618922, 4124795081, 810037070, 4125345353, 3607694888, 770621562,
            632370000, 674117742, 765128043, 1173992895, 3674396824, 2526304384, 3829190715,
            775137268, 3029466301, 2820944962, 3085195001, 2621306925, 2125299873, 2214882737,
            513034349, 908126002, 3167919913, 1027217000, 1178561950, 511143524, 985493719,
            1655744880, 2033154762, 1280998575, 3453548532, 1570014775, 2222753891, 3327333588,
            2154641854, 1735888068, 1817079241, 2722314559, 1762274850, 1167853659, 2392945518,
            196410344, 2937981909, 1154226053, 4232721914, 700255979, 2415771290, 1791869427,
            2898400848, 792514249, 3599536910, 3227896255, 1529237487, 2400490460, 1826096927,
            3142666937, 2785566731, 2723268878, 2079318688, 4034165354, 958450787, 1749435564,
            3977035150, 3576602733, 729666328, 72221437, 1176330488, 3920869422, 3994801636,
            2732985628, 1490190123, 1354581325, 568851886, 2669564985, 1060093818, 3170119834,
            3226579436, 764079092, 1913198946, 1083783781, 3261947326, 380306393, 4209390001,
            2158391567, 1689298052, 1046236714, 442820313, 2757723527, 3143120919, 4267903837,
            3015944922, 1574598807, 2206549514, 4259852673, 442217304, 1830366479, 813570743,
            365463383, 3543466921, 3138253410, 2683541076, 638143597, 995642363, 2769878603,
            4107223702, 3481953453, 3133262725, 3674261732, 1033288545, 3669732737, 3503245892,
            1530461030, 213751298, 1629050977, 375601856, 3395574573, 1885945502, 1714265586,
            97662695, 2156915149, 148240149, 2725333961, 1842005317, 2670201827, 3018625279,
            2036280580, 2241502128, 2888719467, 472203886, 1658540185, 3258448659, 2760284408,
            3494196803, 3420499172, 2950901152, 4276940915, 2202303771, 1096785836, 1693282825,
            1017323714, 954195871, 659807759, 2833714030, 2600259137, 2719988621, 1351376955,
            889846153, 1588673264, 1457310902, 3149228705, 1509379998, 904691368, 3109598917,
            2820147688, 2098775601, 556249763, 2898076541, 1003774548, 3470807399, 552610489,
            3623150789, 2629316482, 1385503428, 3451073512, 997009862, 3235185265, 2184096609,
            3290579922, 171379862, 3550817154,
        ],
    );
    // - n >= MUL_TOOM8H_THRESHOLD * 10 / 9
    test(
        vec![
            3325828688, 2222275271, 2251341024, 536425450, 4031282462, 1974932579, 2802802687,
            2494652102, 4097509118, 3929993760, 2523272424, 3991617974, 1383705144, 964369450,
            4021179190, 2350994587, 3952940920, 2309645523, 4239355231, 2251911575, 101330509,
            33531098, 3923438820, 3326248612, 3928986797, 3712073953, 656648130, 3992081850,
            2462208525, 1232308887, 3597612252, 355901660, 210877106, 2675099515, 3078639028,
            3753546437, 2179616483, 3320544043, 4275906121, 925959266, 3421489454, 4036352291,
            3528964180, 1961636686, 4029362615, 4282725968, 1508419584, 454322447, 1001429999,
            2787320225, 2760755036, 1794472322, 532278069, 1568975823, 813350455, 1179027655,
            3449886418, 2662435721, 327795406, 75354904, 1665124401, 3382081813, 3780617120,
            608468859, 363983678, 3351287669, 172973237, 4290451185, 1931365909, 1235521618,
            2978614510, 2419031024, 1057701246, 1493733706, 2567636367, 1775786991, 977238213,
            375184150, 2098442552, 3863490194, 20320547, 2460513991, 3622662467, 3550564454,
            866710499, 1402820025, 2079884728, 3083812684, 200095309, 3509895595, 2058450016,
            802356608, 3193234061, 2967885003, 3592035474, 2888252198, 1738389872, 2122697745,
            2521126706, 3044563659, 4010640449, 1357672322, 1713997889, 2361412719, 1177264933,
            1279061404, 4226778687, 2234083457, 1990126293, 3249778747, 2856364385, 2539332735,
            819602532, 1941236090, 148789327, 489490234, 3063005023, 3823539446, 2078730493,
            2897346932, 377341414, 3559412328, 2789507932, 2744784480, 2083318692, 473338846,
            1592405981, 3262908934, 3803857538, 2113012145, 4218812261, 2611838077, 2967681456,
            394117396, 891428411, 94508804, 2461818431, 2882613800, 408794286, 1367921120,
            3936232028, 3371928555, 13284201, 2693865930, 1445840494, 62166267, 1950431070,
            903939097, 3181498906, 2135209486, 2379836866, 3164065611, 3089382166, 335815175,
            4115282369, 2843606362, 3030658374, 1981636253, 1913879380, 2497243978, 1058325216,
            1962967322, 2964668797, 316843966, 787872107, 3686059070, 3432414174, 2952226003,
            3159358517, 3532762764, 1103507717, 3261418139, 2938324789, 1645903335, 2648573661,
            4131792913, 2593029209, 466073693, 1785143940, 671647647, 3138504192, 2192173137,
            2335379487, 2298646507, 686675623, 3214302238, 950130948, 2421785284, 41585098,
            3553768120, 3417442608, 3058443407, 4154557833, 1412178160, 341659142, 1776579622,
            1881568208, 3733601960, 964911720, 3954636144, 2069281179, 3720759380, 3881878556,
            3401603350, 2025509639, 3992322872, 3417427856, 2078371992, 3655861835, 249491133,
            897420882, 1663580203, 4250858290, 3783171030, 1618859415, 3059683957, 757177328,
            2284779051, 1397702306, 1846926211, 1628089495, 1310971676, 638095836, 1920389982,
            1228576446, 672122730, 1853316843, 991512951, 777308841, 184904686, 1065934342,
            904702614, 2120865508, 3991071908, 3089750336, 2087140090, 2698642164, 2746911990,
            463823593, 2169525572, 3914335068, 3895138434, 1167324714, 1405364247, 408435330,
            1048149429, 1549143044, 4160291749, 2958661271, 135472469, 1664575385, 2395824355,
            2799959090, 2058237486, 1885870506, 1018852433, 2464874288, 1472031882, 1920049008,
            2124900114, 1894970634, 718935470, 4135614331, 2870932762, 3046996347, 1175241218,
            3337056067, 2823776676, 3880099332, 317500197, 2412894185, 2360582270, 1748097135,
            1117101645, 3999621027, 2508769432, 1134413571, 297583666, 315312277, 445840313,
            835956286, 652883235, 3449607625, 3112977447, 3858652947, 242783714, 3248680155,
            3743017765, 3771708806, 2121953660, 169624798, 3334753694, 1024518980, 2123582216,
            3512992458, 181383076, 388250535, 2264863429, 559757274, 136676916, 3010225408,
            3261874572, 3254590586, 2353758191, 1769226054, 2574994323, 1412248375, 2194959705,
            3161480726, 1173862339, 3060136500, 2067215193, 1559995937, 4097665144, 2227105594,
            57455397, 2781855816, 413139676, 1262905949, 3710184455, 4204785080, 3841061262,
            2600467865, 756074612, 1833352799, 414287525, 2487131325, 225976232, 2868255206,
            3149420271, 2893863898, 2674087497, 4201275735, 3557697552, 2539732703, 2075871670,
            3238490486, 3281311242, 4105032293, 1061668000, 3948441870, 2632408910, 3965993308,
            2773976056, 3997099675, 1044008934, 643256924, 3662238906, 2364413528, 1890816109,
            653190236, 1449201495, 830290176, 1608991933, 1977703847, 3177041981, 41644776,
            1806748422, 46749183, 1969966265, 650370247, 2860464309, 2000093407, 678041782,
            3053985966, 4126712541, 4206913677, 1804281861, 2562943781, 402107792, 3743850770,
            1880600543, 2284692048, 3402413270, 1372820757, 410822514, 365444773, 1581271018,
            1311294256, 2098281541, 3795689061, 2741077395, 1671858646, 779953232, 1441988536,
            4122756260, 3219142143, 2003131209, 3186044612, 3862216190, 2906214392, 2843498419,
            1258517655, 2377662254, 3165185832, 3743620644, 3149688047, 1175443948, 1119755292,
            2586039430, 3583379409, 100277700, 2686178939, 3718199786, 144906097, 893565260,
            2837950407, 3324016696, 682674679, 31411133, 1272850441, 784780622, 3423224341,
            488352871, 4077116759, 26700931, 3657423384, 143033774, 3228467568, 32720232,
            2721507038, 1816011700, 2818467222, 1533431835, 1268665537, 475079949, 3341112122,
            2698588771, 2611521940, 1972051120, 3510532417, 1731958257, 1183168565, 2931876429,
            434380301, 1295823, 1351376380, 3803168520, 92207634, 3345836213, 549845616,
            3480463245, 2630703140, 4194446958, 1396974448, 2365711823, 784004900, 3467957992,
            3284726834, 1131138204, 4158973228, 2854147296, 594452704, 1221140243, 3732311917,
            4211451998, 4201866268, 538219794, 228992404, 463147167, 697753586, 3806158550,
            1256983333, 3561565763, 456221521, 2524923884, 2331508852, 3915487198, 2219329827,
            1899461941, 1568614606, 3436289344, 4105213304, 2685855293, 1185390866, 2047983481,
            1781737500, 832012199, 3513225593, 3319838870, 1020767641, 198457781, 2727609160,
            3813889481, 1016920097, 666935018, 2505214320, 3346073559, 2415444335, 488746607,
            2468423301, 1532367906, 244477084, 2784080007, 3392011412, 3296350838, 3408789716,
            2601539997, 65837806, 3808173608, 2740909858, 313044653, 976945537, 695283633,
            3903895508, 1256467142, 1796137849, 2299402928, 1030847133, 491304484, 1618179568,
            3575375798, 4084273125, 842560443, 860483107, 1477578512, 463362438, 4047759459,
            4170469452, 837933293, 1510207028, 1002537167, 1913407491, 484611587, 72194857,
            3758699609, 1038836152, 3702790723, 1602454353, 3156992372, 555460491, 3102910892,
            4107831420, 1379557424, 109354775, 324500806, 1309470828, 973263813, 2913644320,
            2556229913, 20916645, 3990007622, 1827025683, 894739570, 286790026, 257745524,
            2888654551, 2829856014, 4024894743, 2592875773, 3482365150, 2197208367, 2008450133,
            1021127724, 252619891, 2171006932, 907971982, 3664509602, 1585606268, 3112067844,
            1006461549, 3762990233, 801101738, 2502145896, 1780081016, 2853239568, 366864228,
            2569203482, 376803343, 6955716, 2316603781, 1631421558, 3996270452, 2436912056,
            479717229, 2577014660, 3814374282, 2295304310, 2205472042, 1941857703, 3835035113,
            1672360810, 1473045908, 1338619978, 3516219559, 3903963113, 1518163632, 75200618,
            1452199469, 1006778265, 2872308953, 2526575667, 3217010709, 2082180265, 3503205635,
            1689527017, 1366271253, 2419427984, 3090351720, 758562413, 2893201179, 433319737,
            289828739, 2697292032, 3319507333, 376413377, 1856268054, 1102691038, 406692403,
            4199659161, 1030925701, 2852188477, 2282715864, 1222230217, 3204793449, 906571587,
            4246852852, 2109467439, 2758293720, 2590683410, 426954830, 923580140, 1720968355,
            2177884542, 2700069460, 3414003144, 882029796, 673015972, 925314297, 394315706,
            2405566770, 1356681458, 3704192657, 2420160690, 105120919, 4135620617, 1146779249,
            4215347588, 3808058087, 848912652, 3166226285, 2673900140, 2633829134, 4227829168,
            2976863082, 2756181703, 3969932852, 2310706349, 2623195640, 2975200819, 2683387458,
            3670713004, 2711197327, 1005309237, 2656065409, 3585314671, 1067912540, 3244103997,
            1167067071, 1147905270, 3508458421, 4173233272, 3528308485, 2229102062, 438675126,
            482013728, 933288742, 772078253, 1545301066, 2048214401, 313457524, 1830857166,
            4210633504, 4033732262, 1784314122, 1876717626, 2898790181, 1714304237, 1513498365,
            466291039, 2125921729, 1375952994, 1759597111, 3485421103, 988486613, 447644730,
            2659753292, 800671165, 2297304303, 983566542, 706647696, 2025875912, 750248929,
            3818471532, 1823442438, 797944894, 113229111, 881004126, 3388066422, 3504793499,
            2531563354, 776681039, 1855992120, 3089939582, 3975063378, 2197425240, 1859908221,
            1050864539, 3696699978, 2797794284, 67143977, 1476449198, 425254009, 258997293,
            1600017642, 3718913920, 2057245496, 376988171, 3765611739, 1761878897, 851389543,
            2221681000, 173564387, 543006829, 4033266201, 3495318826, 4944235, 77704986,
            2003516763, 2947938566, 246616380, 3811434913, 1498883939, 2662747924, 3098460145,
            2789081063, 2011354011, 3559521162, 2222938111, 3518312618, 2907191527, 510822818,
            236641464, 2906121970, 2726617494, 1403621963, 2506337633, 1393649778, 749094150,
            2141211894, 2997803954, 3536825181, 125817965, 2928546113, 3962138194, 3488998295,
            2977774519, 870261554, 3145088767, 33204035, 417892208, 82457330, 3761918526,
            3185089606, 3717474816, 3759935038, 718221041, 2584890356, 3125768610, 3128296046,
            3921989069, 628371462, 2854153931, 2322749823, 3560748642, 3598001904, 1525423357,
            3489200883, 557488134, 67075987, 215443365, 3960989574, 3610937359, 1994915328,
            1322852160, 3224593059, 681359565, 1428522926, 1509511379, 2910418227, 2950733843,
            2264582595, 1066502645, 1270894973, 1982059326, 3466419533, 1247820141, 2814957983,
            3674243179, 359593364, 2852856092, 4250958308, 4209800488, 3182992525, 744053024,
            954527614, 1920812186, 3420696600, 3538429076, 1174859849, 1575837064, 2476180617,
            23409444, 2303547985, 3866558298, 3511588040, 2073420498, 2694981488, 3634968032,
            3148001223, 373812485, 3760234630, 1989707843, 553246242, 2798142055, 402011062,
            2572428035, 3149472471, 2641777212, 2341709250, 659173885, 2636588362, 465418139,
            2855449653, 168901226, 3411187454, 3648538254, 1914534945, 2021809126, 1765630658,
            4210072200, 672743231, 237435144, 1466943252, 3665058313, 1360883147, 1208845855,
            4000190916, 3503509301, 1932609745, 2525291982, 3149954146, 2863621568, 4103714853,
            376631824, 2290852702, 2496109340, 1639920015, 3769422630, 1510557202, 3513608516,
            2550439168, 3553460045, 525038818, 4129480327, 1125484353, 2835875384, 4179933318,
            2509396027, 1732889211, 2751886266, 2147307212, 387970481, 695149207, 2541343790,
            2839783121, 1160657192, 3963440385, 2789882800, 2391592284, 2916503838, 3282735897,
            913624563, 3175831231, 3466284576, 46459397, 3590165825, 1395008049, 54895546,
            4114843666, 75582093, 1715376849, 2361175117, 2492505326, 473253401, 3209625236,
            1910683327, 2303707709, 1595992099, 1067727453, 2175266120, 1187538095, 3338022275,
            3861034887, 3646604070, 3901278763, 1297481444, 1740087966, 1713911231, 3579924993,
            438772896, 2847030160, 1091694853, 1462997245, 223788055, 378112136, 3379831769,
            1612071297, 1026371651, 3588586410, 3550643691, 3517421864, 108677329, 1708329683,
            2705550295, 536532842, 1609412940, 2350366954, 3999999603, 3009090634, 3009405637,
            108513089, 433572330, 630309072, 768308286, 3130148610, 696549190, 567069842,
            2991858807, 3544523483, 3196876723, 392972829, 3810464826, 712888215, 4116891580,
            2898306195, 3552494747, 2628398237, 4210499108, 103633346, 2236449210, 3834327177,
            3420048764, 1756265114, 15054403, 4040225317, 1594762873, 3042414357, 2480439624,
            4223576462, 2518287349, 1400158860, 3257170441, 1299629753, 592407606, 183091721,
            3221878045, 312797076, 3985484678, 3630279986, 18819047, 441536758, 2381837163,
            4169471868, 2170394338, 2181312095, 2448805236, 3378100655, 2844803458, 1168647372,
            383072198, 1590554568, 2280721135, 4116487539, 3850200505, 289261506, 3357742687,
            1915184525, 2779238680, 1251240337, 4273338571, 2883259275, 233548562, 4271818632,
            733727140, 228666724, 2488848008, 2996503489, 1445047215, 43172552, 3015066792,
            4234864747, 2683528981, 2327797015, 142567097, 1677432447, 2182227962, 3561493714,
            616143851, 1146197188, 4043856658, 1532480004, 3914042242, 384129295, 1027270960,
            3759841559, 3306756270, 280945974, 2272666526, 1787905496, 1356479606, 1003577488,
            3182186379, 2080620780, 489613844, 2145748221, 3602256409, 4039936254, 4230034790,
            1041216417, 2869721346, 1061058447, 4187665645, 2056653701, 293593624, 1671090495,
            3743958178, 221756712, 669423751, 873593705, 1984521258, 596253067, 1854401523,
            3641730818, 1795122963, 3434366685, 144714542, 612800121, 625529195, 2115632120,
            2458286503, 2991075249, 4294925998, 3484272210, 413395376, 2841974813, 2198382581,
            153452909, 2362633408, 1029933732, 365559147, 954593410, 2340863943, 4224478519,
            1711517777, 2382640557, 1471136977, 1243256182, 577943866, 2877182972, 3120699107,
            2099481569, 4027415860, 972360440, 328292954, 2580374611, 1126114830, 2447790959,
            3167058778, 52949279, 1503347494, 3102470619, 2850741785, 1440074775, 2983184898,
            2958779770, 1423341775, 2638966039, 3414709326, 1746843145, 1962096213, 1231942872,
            322282459, 2520802915, 1397156397, 1136524931, 1077984387, 1082573606, 1369475996,
            1343216443, 2853222460, 459314534, 2082494332, 1868894956, 2863227388, 1267699732,
            3838811811, 2085654049, 3478261558, 801390064, 3020980804, 1592039288, 2043252793,
            658119035, 351482607, 365151551, 2321964000, 3105703149, 2651451296, 1332089903,
            203189386, 4065665344, 2087988078, 3509982235, 3663789691, 640321038, 126379371,
            758704611, 2976133119, 338240358, 2847694981, 3405531815, 3501389852, 365915081,
            1893710987, 1017798137, 1329052770, 900124839, 593520276, 1673088115, 2837108009,
            2725846942, 2370117674, 2364056627, 1588943446, 3447621374, 4188928348, 3651554937,
            3256606153, 3302425960, 3080093332, 265589115, 4231992887, 2117604928, 2355480969,
            1006273543, 2284486530, 4056290811, 4091031679, 4079416675, 2288531059, 786357019,
            3958013190, 3469028967, 2618870759, 2582243721, 3134347005, 1897624527, 295895331,
            3299510488, 287519621, 1532582663, 3409797611, 2683376466, 4283418958, 1877809103,
            2723998791, 3144672840, 3727310197, 4009525850, 889434429, 771869475, 3566747894,
            2738193100, 813849012, 3911388965, 1279847132, 1685856902, 3082912514, 2067299920,
            1405185595, 2885740666, 2118178130, 2496932272, 4158468024, 2915749138, 649483477,
            4206421293, 2757754394, 3314522431, 2269462235, 3993183136, 3700765328, 1068976941,
            4261401847, 1124755401, 3801610743, 1969610114, 3664892243, 1112960775, 1912361842,
            1030225346, 967360681, 3986502374, 2169456113, 3758060719, 1480705586, 492248116,
            4222874616, 3319928478, 2992291382, 2450502117, 3949036192, 1815871387, 3254185821,
            711097033, 3511389055, 2820383871, 3171069864,
        ],
        vec![
            219605684, 3733847704, 3016635823, 3294762792, 3944405696, 1769872727, 832659846,
            484088999, 3505517372, 665090353, 981244631, 1379204518, 2636388743, 2646535623,
            4245306143, 194502210, 883787587, 458392293, 1851531468, 3012939792, 3916075164,
            2268609087, 1167270783, 4176306316, 3743775706, 2950832151, 3594959653, 2285984635,
            52893824, 2172982965, 1248959379, 1470557476, 1201448597, 3375520425, 3748208049,
            1257559380, 837239835, 4133304044, 1858466116, 560253153, 3678427423, 455815704,
            2048555203, 3908746885, 255785748, 3209187195, 3421604835, 2632159689, 316833694,
            4281401440, 102090580, 3721736010, 266287611, 3373488642, 594309134, 2738851237,
            1684934520, 1276732189, 2903617658, 3296557081, 3563767242, 2176348197, 2313418457,
            3233080717, 2735279615, 394359834, 623713995, 140049212, 498519169, 1402029947,
            1678680803, 3015892739, 427600690, 1125893168, 2179497262, 4057603178, 3496071310,
            2899325724, 3677322280, 3804953038, 278886520, 645458896, 2486879584, 2394473074,
            3850599181, 792951084, 663635668, 2927466603, 3639001969, 3606253415, 508580584,
            396794482, 2985523809, 1445903235, 3313779349, 1676477042, 103850445, 229595348,
            703720916, 2578553830, 2458841523, 1933251296, 363644571, 645442292, 2557064683,
            962374614, 1700508145, 3794324048, 2584976073, 236173708, 3764420274, 279923540,
            4075757279, 2235622749, 3191608858, 4006438835, 1908504238, 1404730391, 2893611100,
            3418919975, 2761500606, 2538142606, 3656578307, 919774545, 1445376035, 3424913709,
            554731787, 2870801108, 3893878559, 2515735309, 4049168009, 2736235180, 4046071341,
            2836164363, 3926498800, 3814849323, 934280090, 89568632, 2294387373, 3912402110,
            2118681178, 1351750387, 2447640447, 574461143, 3931298943, 1794049409, 825791603,
            3855614361, 3804227287, 1483097620, 4259972568, 2252285343, 123276475, 3161686393,
            3218037245, 256027861, 809544208, 2486313226, 1050383855, 4237284978, 2694106279,
            3038308664, 3410660322, 515412998, 3869866599, 2994392437, 3269069042, 3947466847,
            2883348919, 4052591666, 1863977555, 188271850, 4149287894, 3890111279, 397365955,
            2163011790, 3701205263, 2638690803, 3839607326, 914036595, 4046887696, 1336652496,
            822203045, 2784897683, 1265234986, 1784732027, 1447844299, 625275839, 82219488,
            2987602621, 3044052434, 1181776664, 1530351091, 1110986421, 2060624013, 339066774,
            1590502368, 2317588177, 2244585652, 3492887264, 2212680022, 2640475326, 348926579,
            3246343174, 2561437332, 581962653, 2221706178, 4069194219, 2284871124, 3048622008,
            373685125, 4025211847, 1292047419, 3120672305, 790731263, 274475368, 15199343,
            1922359621, 1749773031, 4241521715, 2479848827, 743662349, 2151293746, 49164230,
            344935070, 89801179, 142306069, 2327816182, 3527604729, 861275048, 3754133025,
            105007617, 1606758768, 3411783480, 3900766726, 229816895, 968892827, 603676768,
            3067574405, 508084800, 4163966682, 2520459787, 815626761, 3750642090, 4109045197,
            4021986339, 1510247485, 3001110466, 2167226521, 3591300424, 1416732703, 910590371,
            3408174912, 3155039041, 1139149626, 447404795, 2716484878, 4070167463, 308857448,
            4282968848, 3099451490, 2414982180, 316232599, 642439731, 1545686574, 666961383,
            897959935, 3374635967, 1881209465, 1374103300, 642609814, 502781940, 3682373645,
            2489773845, 1283267440, 3894229942, 1941929986, 3934687928, 3333644753, 2997423793,
            837943158, 67336216, 3255568419, 190224485, 2837663345, 4207768659, 1708933825,
            2772287530, 1154500232, 1693433014, 292641434, 1754396219, 2542674547, 1190079584,
            2269894063, 2721853439, 663916801, 2362454970, 731922427, 3742164411, 707311168,
            3046591691, 1748326630, 1845653289, 1016681698, 2994353806, 2735719342, 2704445240,
            2854070151, 1607062766, 3744430615, 3544224665, 1886417222, 3425058873, 1609422240,
            542845843, 3147109760, 1424468935, 2839056308, 2918717971, 2811887991, 2790495639,
            2132641907, 792891625, 3102872968, 1651448144, 998081822, 979132706, 720547406,
            4025787009, 2735580435, 2457487375, 1065238981, 3412254354, 215131432, 3048596830,
            1669254548, 2090345212, 509499351, 3615558193, 4162117786, 1536698846, 1534147203,
            1923724108, 3528335633, 3395028864, 404544658, 4167466929, 1214045063, 3598864251,
            1326576655, 855338193, 3368941689, 1401909859, 3124328704, 67082155, 1595352370,
            2689198649, 770470257, 3065267028, 1211541561, 422514087, 536466990, 995741725,
            2502422928, 137873536, 442872337, 2288928710, 477168639, 2527159632, 2500546045,
            1397043975, 2205863722, 2911856171, 3714388841, 1549554282, 3753612677, 1890046352,
            4039530596, 4146084646, 1706325322, 2454110812, 211090074, 3163995441, 1637968681,
            365653242, 4282639641, 20611554, 3054560222, 1999188492, 3488521981, 2868372811,
            2973275585, 796515146, 822994172, 1030297677, 1588520622, 4228102402, 2708306471,
            1208006335, 2078217033, 2586553571, 303653553, 2132325166, 747687456, 1747825014,
            2837410885, 438830471, 1600784035, 4134092721, 1506389233, 2127970290, 2668085318,
            4089024642, 2575734777, 327185130, 2469533410, 3597704057, 462560760, 3152215881,
            661472818, 1938255716, 356786526, 3439418576, 6826038, 4187428395, 60261767,
            3087172989, 1450929937, 3651563872, 1589078139, 1677344341, 1470806427, 903804397,
            441015765, 1272755045, 814260009, 2653285733, 3860945219, 1455674283, 1096658320,
            3516576877, 3023176095, 3479966062, 3407627614, 144957111, 2468468573, 133078933,
            860997956, 3491202246, 1622521278, 1232931671, 1342648914, 1925040885, 4195720141,
            3799076149, 320357412, 1260627691, 1901795898, 4006861680, 2967730799, 1348760243,
            1207763786, 3372249393, 3370014578, 2928367142, 1013740101, 3800825723, 3853764948,
            2540691600, 4288140609, 85729198, 2054750317, 975078435, 1603168880, 752297715,
            676265726, 3745588378, 1190456489, 3199796255, 2568311924, 1636039660, 4172418324,
            451873022, 1488096996, 3888837104, 2380625095, 1503612236, 1174254768, 3690690154,
            796118737, 950243347, 870818727, 3575441985, 2933836937, 4012643254, 4154690232,
            313943165, 1194823142, 112872260, 1583856748, 3091518525, 3848126856, 2316319296,
            1177342115, 1834090, 1312844134, 562564763, 968697255, 2625292379, 1560369157,
            3793042995, 1360344305, 1561892888, 2561644342, 2880487899, 1329308678, 991881816,
            408459653, 2048208943, 798151371, 3438869282, 3477838848, 1731589585, 2405599989,
            996289971, 2339728423, 2712816129, 3199519522, 102417245, 3737926264, 2499734949,
            4289682039, 2043689414, 2819686077, 1679268219, 222221440, 4008029847, 1560387789,
            1298710311, 1243123895, 158514034, 3873321117, 3004642495, 3872001585, 1663566476,
            1107233170, 293716059, 275162247, 936579960, 2990240463, 1926069351, 3296513110,
            554621834, 4144199052, 1392609836, 1034899022, 1208790097, 2807112109, 3964061416,
            1653167562, 3742290159, 2346753191, 3650591545, 522876039, 3945783347, 3065661379,
            760781159, 2219622460, 82469461, 2887087892, 2552061061, 3838920437, 1162671243,
            2030025601, 81626876, 1751905726, 2166036662, 1557608721, 546818324, 1585093311,
            3667292391, 3522898667, 2321723183, 1047786840, 2621628330, 2989416129, 1420910156,
            2819893717, 41408882, 1090794692, 254861895, 1531947943, 409858928, 2688575425,
            638494402, 1137167657, 1407867246, 1512173911, 1408432723, 1473209607, 774943403,
            1286624130, 3425873895, 3620263656, 1909123130, 2551706705, 1549113484, 1091256254,
            2885486188, 761917899, 1632560671, 3199850510, 3435108107, 3984491598, 2758162447,
            3408358152, 3614032341, 960132339, 3328987609, 1661534291, 3165532505, 2611312032,
            3366533473, 1979920417, 1312551138, 117282700, 691866345, 4139053046, 3105013808,
            2777561811, 2635054512, 497784664, 3126213089, 2216274215, 530791472, 4066290432,
            2800488077, 636174841, 3629392743, 1077206808, 616054960, 3838079466, 2365491331,
            4280387869, 4085608937, 2387284685, 2905568474, 3687210908, 2676786135, 2880576182,
            2272479710, 1199932595, 2261850657, 3804274819, 3762551932, 1458028870, 2897463116,
            2416202598, 3392536019, 2870202002, 3613666863, 2249394797, 2992036599, 728534566,
            1120088649, 3979378545, 935693277, 1031467147, 84108903, 746798643, 627228099,
            2800690251, 2173469180, 2830223765, 3137722777, 4178739722, 411466414, 859751380,
            1559439628, 3694498090, 3260343774, 535054266, 117084126, 3399243337, 2495976075,
            4076251817, 2926801036, 436287733, 3705696306, 3667196595, 96134495, 3729370692,
            1515068169, 264787485, 2798154317, 175985133, 1313840608, 760864127, 3967639961,
            3959432295, 1635932259, 3331385869, 1070261894, 246190898, 982482783, 3854791453,
            2982674210, 502540469, 1076671794, 3188598990, 1279447641, 3784328740, 2394101763,
            3346901062, 537444017, 1055795404, 3549995587, 1164547440, 1373235322, 371022910,
            2435082373, 3557245778, 171522386, 1140771969, 3265269810, 2780244587, 1553185664,
            1890478317, 3220213095, 1312302522, 2431923700, 1223485266, 2282493771, 921148941,
            3067060161, 324175533, 170250519, 570500159, 1035903244, 2479701867, 255677361,
            2023064938, 1790936466, 473377155, 1918210065, 2072725791, 616083034, 3885053377,
            662826718, 1434245870, 1910220894, 2309848114, 525544021, 356039132, 2359013744,
            4239245004, 4026803267, 194151528, 795233412, 305493034, 3267801916, 4080918986,
            329727861, 2899234059, 1357709403, 3251519907, 1938363589, 4131025749, 2594354892,
            1660726670, 271386306, 499198400, 1339562618, 1867450859, 3440407368, 1403232131,
            1272086132, 2465102747, 1633075747, 2461437511, 635308997, 681776567, 3489150051,
            220625570, 2311229261, 4200050354, 4050773484, 370413512, 931753989, 3307172724,
            183245723, 1765157394, 47788463, 3937773236, 2598232722, 3424882189, 1276600447,
            1644552616, 2626308042, 3268722786, 3810874590, 1058205206, 2654249153, 3483559388,
            1838428373, 3023229213, 2148907687, 259958622, 2710064692, 277383554, 1415212012,
            4053351646, 1877209663, 3049048356, 2164707394, 811696118, 3639050029, 2733433970,
            3499435557, 84856309, 946720455, 2593206349, 1653444547, 628472620, 2602758976,
            989666944, 1666426206, 4093401526, 2013618092, 20022648, 3142131018, 2920750232,
            3012941536, 713849141, 217371192, 2291693390, 3420908933, 840008593, 509521091,
            2216030985, 762663986, 1144893566, 1669626401, 3553089528, 3848788572, 1322528707,
            1139160791, 3245062559, 354845554, 3395160786, 3002919588, 2479702839, 289294866,
            1247980658, 1768709894, 38829267, 2796896988, 3280594127, 597391183, 1674125,
            154337128, 1219465231, 1594491341, 215253014, 3941260773, 530738196, 1659968693,
            1997144375, 2363494810, 1887727442, 2372279514, 3569468084, 704702032, 3254327350,
            3274665109, 2746129291, 2402740279, 680910327, 3196302449, 1515184422, 3018121467,
            1533706515, 2154506929, 4271801874, 276946657, 1454657398, 2035806618, 2347369271,
            4167466602, 846913372, 678827116, 906512673, 2989625336, 3380365559, 1817466418,
            746879498, 4161207963, 2652566219, 3997761296, 1729126521, 1915492417, 1387691720,
            3792948085, 591813962, 4166608463, 85460956, 2019829952, 1093520400, 2115527596,
            917437663, 1506881666, 2919904171, 1226740800, 2827153204, 1804490316, 3305326376,
            2228657260, 4144079407, 3834647065, 2630139426, 1540825792, 2224653587, 940767108,
            3878245257, 1227320069, 3532608478, 2225819731, 3888521056, 2837907080, 3526288423,
            2888635755, 241668691, 2791059392, 1425900065, 900551913, 2183647741, 256669272,
            723956303, 1186667535, 1960634662, 438856681, 37239087, 3623703669, 2346924024,
            4084706232, 3401696786, 1070387337, 3327464499, 2530115640, 4095803022, 1354739657,
            1132191762, 400698935, 3582112175, 3859178123, 3682980963, 3946109014, 99291864,
            1473874678, 872297294, 2480703905, 2587482474, 4621945, 3807688592, 1223705190,
            3415437317, 2878715234, 179753873, 1451307698, 2351373338, 3977117882, 742110068,
            1159308459, 3051728070, 1114874234, 4063286511, 2958632124, 1126682061, 4057528706,
            3462397411, 3615907500, 3832358400, 1520557386, 387566035, 4181990938, 3576980813,
            1763909198, 3546068004, 2178077355, 2942665537, 1777408679, 3972359411, 1398692519,
            1334374308, 109850799, 3967277062, 2696558472, 3920736437, 131453960, 1927970889,
            3285016317, 1862961941, 1740391758, 4268655537, 3710775553, 145796565, 246130446,
            1603042751, 595310227, 2018018140, 3941725849, 2484543949, 3362738863, 1417106401,
            1903645571, 334821664, 3294839018, 757433484, 2445590565, 1336706929, 958713100,
            3159315785, 391507292, 2052141380, 2910107823, 1346035310, 2963367850, 2254143762,
            2499969819, 280296612, 1528879389, 902597237, 42322638, 2748873147, 2698107871,
            1431611471, 1665375338, 159237274, 78229197, 3026759027, 1323534075, 3917805809,
            2391056398, 1505295912, 2068279224, 1338863823, 1696316814, 2515636746, 1277950518,
            3450429578, 2101302290, 3205130735, 196407224, 188726922, 3038097197, 57783385,
            2383491391, 3407259298, 2799181690, 1609954168, 4123135002, 1545697370, 3578629713,
            3610539005, 3582171858, 1278392959, 3967450424, 1821082540, 2433887435, 2290405682,
            4274636433, 3622463783, 1758441609, 1745493987, 1614890805, 392482483, 3736377100,
            3290414371, 2674564964, 2735379425, 1246652091, 3449848348, 386473850, 3866749153,
            2138760498, 2782728745, 1548369266, 3567061001, 1675699086, 2780788787, 506858834,
            2071383164, 4047965152, 1464258966, 3695163491, 1868717279, 3813178694, 3237288978,
            986068490, 3435148318, 2122044747, 1246656336, 3896225324, 1575964969, 4213793464,
            1109969802, 1452144015, 3212129159, 2302557278, 1502824613, 644906755, 2914538770,
            862243454, 20971484, 3662171943, 2417948473, 1231550572, 1646849971, 853366230,
            1122931640, 1756293226, 179544062, 4215085567, 3606104577, 1712071933, 4088212811,
            2878352034, 3938556619, 3688177756, 2048888802, 999798004, 2557974383, 1010120016,
            1928428516, 3117908667, 37540987, 246311472, 3280437538, 1881918816, 967577101,
            767831376, 76432915, 1933946275, 1880512603, 2935718302, 2245985548, 3963035840,
            1648734611, 2660142479, 2100871578, 3438105035, 1188821535, 2922385250, 1576180774,
            1250780564, 16622501, 1607339944, 1002015015, 1956060360, 2787486043, 2001196000,
            79889482, 2303305799, 836938799, 2987765730, 1277324921, 2034841065, 1879315225,
            2747596182, 3859208697, 2145327881, 3726096234, 859087533, 3800200229, 2564257259,
            84263127, 3662504708, 3381867668, 2731347057, 2864062630, 426312130, 3193065885,
            1387307327, 4214426077, 2162510558, 3066192340, 1444678925, 1828433112, 2030683995,
            1082098839, 86577056, 1337431284, 2125134795, 872245494, 1176391120, 428884772,
            3304879081, 980353105, 2336105576, 3729442548, 795317116, 4102415034, 1559419254,
            2186254311, 1543348898, 718848601, 2040045237, 3036429816, 1652063812, 1902927644,
            849332164, 3237344326, 1209554830, 1529964693, 3810905532, 726503750, 2332940752,
            369345572, 2173720310, 748171910, 2703721711, 4016901944, 2494187032, 655988358,
            2624619843, 306589730, 444322440, 1527584196,
        ],
        vec![10; 2458],
        &[
            14315584, 3896075743, 1720456290, 1860987977, 2435033617, 3283815521, 2217962614,
            3596711492, 3391485167, 1542534513, 1934312914, 3597773264, 2544643458, 1148491463,
            3826173723, 532566836, 2635112328, 1797080596, 774807823, 998727897, 2159932141,
            3789188038, 2274360197, 2025617756, 3512355374, 3159634613, 2709146823, 1381235091,
            3610000364, 1734574093, 2844675590, 4140474299, 407090994, 4164501520, 793085329,
            4062427786, 610657511, 2840989864, 251486974, 234324123, 4278826782, 4151036614,
            915714573, 163464758, 588848332, 1721190312, 4005707399, 2749479472, 3315373811,
            2305992663, 3034658613, 1026628922, 3044818018, 2078949981, 1076394351, 633172928,
            919810481, 1844153183, 47796503, 3838473951, 3914821415, 56916074, 4224374707,
            118214008, 58206586, 182773179, 2277119069, 3833074478, 1426405443, 4012102085,
            3842259766, 1661555376, 4173078834, 2160138718, 2641435466, 2401625776, 4135020452,
            4261341778, 3597435941, 2878333847, 1697222430, 2479834161, 3501940583, 547428987,
            1699694744, 1589714856, 3094092039, 3812401988, 586705178, 1771549859, 3889356571,
            2081044636, 3915524440, 2215969097, 2785923765, 2891035393, 115204411, 3555780024,
            120731371, 2792997289, 2604840991, 3947175716, 2504012389, 2822841753, 1071118219,
            3845128151, 4253970477, 4253952215, 1276169564, 662601090, 2764969415, 404556610,
            4025178844, 3248541914, 1234899686, 927203128, 2307975612, 2491917326, 3535570818,
            963945728, 24837075, 169486507, 613376340, 1332659157, 491080348, 3160651461,
            4114780572, 1838553440, 1987857586, 3430700602, 2158770837, 2614368496, 1084388381,
            1858796295, 1522300999, 2839927145, 2816285905, 1707265160, 2776617220, 16197005,
            775024741, 1662486904, 623200432, 3946294781, 4240745436, 2146661189, 3067562449,
            1836712122, 106845872, 985435189, 739872985, 1460204407, 1119513446, 330890579,
            3254283935, 1656228797, 43939001, 1237958976, 3472042242, 2309388228, 1106182843,
            3555794821, 36926079, 540976529, 773787735, 1327045353, 4226128278, 193584290,
            1239226396, 3512329876, 278414825, 3926266087, 3786237276, 3579952270, 498291988,
            2823774255, 1688087301, 1721289947, 196581659, 83087956, 4112121408, 2559494911,
            2199502433, 1055770183, 2819943945, 33589713, 2283678521, 286507459, 552667298,
            1451047744, 2651022894, 2228146245, 2540846830, 1050328076, 3047039299, 2766587484,
            3691700280, 1049254813, 857110920, 1159205473, 1941578771, 717689503, 569892065,
            3678807637, 2220477814, 1080351362, 2806798851, 3447297122, 2221865362, 930511843,
            3450660262, 2680179755, 2917564544, 3939325714, 3771507323, 3259386687, 1467770684,
            3837894085, 1528988932, 2370052746, 1580760722, 2250263564, 3487457499, 3543162480,
            911819585, 2979157164, 3598196682, 3923086759, 2653629140, 658857317, 3722939978,
            740695757, 629152520, 2170296541, 113912870, 1694943806, 3625922241, 2631571610,
            3862073130, 2092081805, 22919059, 1192280719, 3218239082, 669906544, 2006614820,
            2447850554, 3276882114, 1137509778, 4023609475, 2022005152, 832009711, 1067392044,
            2457787508, 495444281, 4057527981, 2638116211, 1032205613, 654939893, 1867393721,
            4259720808, 3864862105, 2580711081, 2986252975, 3554203437, 2309301009, 213616538,
            2337770026, 794663587, 553040278, 952559472, 1647697621, 1129676241, 1195552408,
            2257179988, 4158151542, 263190063, 394788678, 304472849, 1116700229, 4164233473,
            4138582682, 1331620795, 3961214508, 3206483671, 3619459583, 3942761684, 1127994404,
            3795525621, 1793064957, 842585732, 4265730476, 2213297660, 1058117485, 2424506278,
            1522441713, 3775954625, 4272294225, 4189249235, 1363414580, 3480077270, 3908998115,
            1436206873, 2926155546, 2818708656, 221476678, 1306834349, 1783935758, 3783744354,
            1055993798, 3867370100, 3300333089, 3110025292, 2247488535, 4015323199, 3356815720,
            2045189358, 174230219, 1749374739, 1218246124, 1727443760, 961240189, 2297210165,
            3075205019, 1458465736, 596183329, 883659603, 1101688800, 3102408665, 1868011842,
            1960371344, 1643686155, 1063869191, 2969813860, 3816241530, 1760766422, 2415281333,
            1552385643, 3433917314, 3680135132, 2620866828, 3307471651, 1980286844, 1955169126,
            2687159626, 334855600, 117109258, 1746064251, 3376322566, 3584584566, 3601971429,
            604861952, 2830443525, 1679490243, 36269131, 796031070, 3561439893, 4022657332,
            2985918676, 17127477, 3761141990, 1343371600, 2767901251, 3804378567, 3191436147,
            3852862884, 3016005135, 1779020584, 3175770274, 511808311, 1203545175, 1008529794,
            700155497, 1917226278, 1626847419, 3459828508, 1537231, 2773424015, 388817294,
            3515169103, 474940622, 844323074, 1246271484, 808369492, 3662858240, 3038799386,
            837049977, 694305488, 1015863588, 2925540413, 4159464622, 3677790522, 2662594853,
            3511496512, 2011009488, 1290229871, 641420037, 2128199318, 596608790, 4224311625,
            2444741761, 2082021242, 105250274, 3696760907, 725737773, 444696069, 1930303327,
            641100376, 487548132, 3502604477, 3831249409, 3112472257, 3558333009, 2068118493,
            2883838540, 1151392095, 2621367657, 456338853, 1553499738, 1204143117, 3272252059,
            2070211859, 1217939590, 1139521074, 1196251017, 2724271013, 414971260, 578940848,
            3971160962, 2353021486, 428914039, 1466220029, 1196519558, 1433890287, 3488235345,
            1745391024, 3271389992, 2189565159, 845963688, 2022888924, 1318009346, 3106363223,
            3294116257, 3097893493, 1464480728, 1050593463, 2122671271, 1211170363, 3885668892,
            2267117947, 863642472, 3731190320, 1733566259, 178546948, 2468292835, 2325992738,
            1754129674, 2704515549, 3611719433, 2588929153, 1954312759, 2795607335, 279835277,
            57394715, 2914757129, 828581834, 3849284465, 2023672381, 880214213, 1837624514,
            3173571098, 2422433820, 3834459050, 2009223418, 4111196121, 2821937543, 792678184,
            3501133001, 4215472170, 2198345623, 923662366, 3183651064, 1599187385, 2463494975,
            2989458960, 1408484683, 2889663583, 3518183569, 4157181297, 2205842269, 1585684381,
            391762939, 2611826234, 1196804558, 1572135490, 3975160311, 2275902996, 1069102622,
            1315683334, 3852967713, 3301170698, 671161065, 128192467, 2613903289, 1009831606,
            3749541003, 4268007757, 3092087731, 2140417700, 4096277941, 3660903771, 614384141,
            531790068, 1909595317, 860446255, 3382696660, 1061474464, 3069773013, 3315525987,
            3523354010, 2175878609, 3945364084, 1871031220, 1024296174, 2318503761, 3880021755,
            3483182385, 1136511304, 1854660657, 1351215670, 665018554, 200157670, 2960727314,
            4133284053, 545593913, 1878146619, 4080637287, 1319623815, 1371326109, 1538817431,
            1523233024, 113264668, 129051258, 2403826474, 3690426422, 1876099463, 3478112649,
            2967083172, 3506759557, 409569116, 303290950, 129270085, 2185938088, 1654816429,
            1606016066, 1937302705, 2748203621, 182457345, 4251467861, 4087619332, 1455766758,
            16900151, 965491445, 2811127049, 30198978, 3752372717, 2055927564, 274588215,
            3391994008, 1128217088, 347809028, 3079847934, 2217407060, 425522097, 2570086835,
            2332474322, 3806978544, 185777147, 2699474347, 3590619670, 2292054738, 2754393328,
            1364967129, 3456367406, 3229262504, 3058183589, 3530281963, 1260299809, 2813856498,
            1699609618, 1560668741, 3219892482, 1660863042, 2699447087, 1139225248, 1391362567,
            2863783940, 3505675545, 177973681, 2882782098, 3651046939, 2456664276, 1206225175,
            2580372242, 861145421, 2184732085, 2769725519, 2282301357, 293785, 2972930264,
            2386258075, 3859546565, 1169200071, 889331732, 2826596513, 817584661, 3393024390,
            1855368622, 1827205276, 2387127529, 2630010835, 3209679365, 418840491, 884209449,
            2057532721, 3963456408, 2146511566, 3657939615, 3357975902, 1077069037, 724050579,
            3103467640, 3641072139, 2311523389, 289677346, 2339409410, 593745130, 2263567727,
            138391246, 796328585, 2755156236, 2335316903, 2840103332, 2555558267, 4191162130,
            2041604517, 1454327959, 2762012248, 345711112, 3271322778, 251706744, 1529366113,
            1560060753, 1273744532, 1745715986, 3472355943, 4216660608, 2700388481, 356464407,
            3241914579, 2079624535, 2335385338, 1915268218, 1444165795, 909641004, 3058658356,
            2994984668, 478335584, 1460779062, 3467507749, 1754767550, 1567226851, 1002377325,
            3381130756, 267437249, 2121264436, 253481028, 936579317, 1351715032, 3705651480,
            320805320, 2843650545, 513701327, 4215594992, 917564150, 479566665, 859669441,
            3157609161, 1488471594, 2176388201, 3238827013, 2441825796, 4172095140, 2146136291,
            1038154134, 2320185255, 837531952, 3680644283, 2246178890, 1376172528, 1150716697,
            4062481636, 3411210837, 4222364616, 1565814963, 1958710568, 3912317092, 980972994,
            878840575, 3785073185, 3184461301, 2896839434, 1329190240, 1611142107, 1066960053,
            3030553308, 2739555728, 1219677550, 1319663819, 3807245276, 3038804156, 3975511475,
            3474347779, 1787854544, 601721716, 2066121273, 1574211264, 2629698714, 942513094,
            1970368862, 253914275, 2798995017, 521161377, 4169584747, 1284753624, 1230700315,
            1384968294, 1964859012, 3291627178, 1980345114, 2549783785, 3965567049, 1871888126,
            2413764615, 3746715727, 47085565, 3023290394, 2113006968, 2213295793, 3257150141,
            4076997720, 1957602743, 2506897077, 3936832338, 2584432072, 3577991725, 3315218282,
            3479103036, 1346566712, 2371604946, 2925130936, 1641375567, 2326216376, 2392362615,
            3415032383, 1114138902, 2782317125, 3786087099, 2257642242, 3028343849, 2677315638,
            2362678290, 1186576050, 2383638912, 508976689, 1371440287, 157924594, 578155066,
            3045004846, 1531823852, 4127560618, 193156585, 885851934, 393568866, 2698643032,
            1487115201, 1785690912, 1560880139, 3506823440, 2249790562, 2196069771, 1655354644,
            1502736684, 2846902313, 3224146927, 1069425847, 1484301171, 3937349528, 3672862019,
            27599350, 3951201728, 346872759, 3254389195, 344295907, 2578553021, 923584949,
            3534810913, 2615758701, 638597905, 271997715, 3258597336, 3729321294, 2555257257,
            3160826970, 2162970558, 3448099455, 4067658524, 2977860211, 711143050, 3898990729,
            477305732, 4126099437, 2285319322, 203339387, 2313974244, 3497142096, 3829537435,
            2373684329, 1580063789, 1630563529, 4181047728, 4251813960, 2633137079, 239130144,
            3187401945, 2137440975, 1756486693, 835326208, 1219590260, 9685703, 2802769936,
            2828332335, 3395480672, 159349648, 1474549022, 753761752, 1033399146, 2663571984,
            3293328810, 1708860985, 2775321734, 1290639360, 3469415209, 1932741667, 4131857604,
            3548751506, 3873065925, 1317695774, 3167730054, 2212798570, 963071706, 2239017839,
            1663037918, 2024879418, 1241586100, 2977324887, 3668861062, 1460374273, 1519821695,
            4149671041, 3603428895, 2544181925, 1881748848, 1326890903, 934934583, 4041040169,
            1251595343, 330283292, 251256048, 2781853181, 1285941088, 3857270978, 2172118120,
            2411830912, 4263916993, 1501520015, 1668582518, 4232592653, 879763972, 2835329802,
            2391710926, 2956348612, 1044031187, 2315020555, 832842187, 1640823936, 3687293347,
            3245417753, 1428535947, 3947019553, 2520875375, 1870345747, 1192441840, 3799658925,
            3361841552, 346211097, 2455121685, 883553097, 3147539750, 3584951361, 1998974973,
            3410409445, 1192475577, 1099659862, 3119506960, 2253622822, 1486966019, 4268179280,
            3660479674, 3393207880, 244989765, 3307625178, 2228507794, 380460232, 3068292465,
            4049554096, 3930654451, 2211137620, 2857990237, 2174742086, 406693768, 3903681170,
            2502874185, 337283200, 2457367138, 2750431953, 718191090, 4292594176, 4100702015,
            3730247801, 892058377, 4270730729, 1475580269, 1364594981, 498258814, 1627557888,
            1445371571, 381298578, 3302152308, 1501315700, 1540275847, 1508419086, 2490558448,
            2211045301, 1960092236, 764821667, 3011598950, 2610267528, 1305693830, 1576485405,
            482671602, 939273544, 229216422, 1132377588, 1530307612, 2725258345, 1846376247,
            2496250877, 2069965191, 2820337527, 1422645778, 3199513164, 399431965, 2806995769,
            1924011870, 3052206613, 2926865488, 1616505509, 2747673810, 1857370974, 2194952150,
            1497926871, 3238231671, 3731302543, 2809222691, 1647371626, 4292916721, 3256825234,
            3570979652, 3007380634, 2699365431, 2248095940, 885821273, 2678808130, 3323361718,
            1143883958, 30479033, 2004872945, 3382927648, 2168115449, 3511946758, 278718047,
            4014380536, 1096793656, 1075537158, 2961900536, 3710219996, 2157081806, 4239096658,
            831258008, 3208663634, 1065180774, 1716500895, 2672567069, 1232298720, 2125798236,
            134823113, 3792486128, 3436278410, 2111754152, 942157649, 2586630610, 806131922,
            738294633, 637501765, 1534594581, 80662048, 2778145971, 1982921733, 1684072932,
            3372420013, 3660171935, 2135748383, 2694039659, 377728688, 700503753, 2985608912,
            1084882191, 4151983585, 1926421903, 3882296979, 710544755, 797297863, 1874922381,
            600821424, 3469363770, 2976113533, 3097650285, 3213892345, 4283796693, 3738314537,
            584844598, 1238389442, 328722752, 2261560780, 1963218488, 2722962954, 1970611970,
            3995953308, 1850769515, 3152074077, 1001957645, 2384943533, 3877044968, 1975105919,
            1107878264, 2803250607, 3005721228, 1384517066, 2059492073, 4218640871, 880506282,
            4072429242, 1924150062, 4080336746, 203753502, 2881428894, 1944674533, 2510372468,
            2208936106, 160544268, 2169335406, 1295089250, 1389517169, 601874109, 3482835876,
            995715955, 860499899, 3257518826, 1582699703, 811345812, 865361290, 3350101589,
            2302946008, 526905664, 3463993986, 3786007662, 1461421857, 558553300, 2659196735,
            1878306353, 2120863173, 1521235460, 1668716843, 342506011, 1304470509, 1883246634,
            904885016, 851659242, 618465764, 62782100, 3904388641, 3401400130, 3033132634,
            3695267353, 4184494689, 2096681402, 2136444941, 2379668661, 3570388220, 3650934881,
            845257826, 1658386111, 216440227, 1448252242, 3867938137, 1041551940, 1271424127,
            1694854790, 2618966358, 1212830768, 2944316262, 3255928108, 1311149209, 1634132716,
            46015894, 1762774538, 3731249278, 3730743730, 4122659132, 4259690078, 2131268672,
            1847903623, 2590863134, 1598577933, 2201759887, 2734226325, 3886633845, 673785124,
            1543386385, 2392366384, 1010403234, 1157937717, 1379471091, 2428469664, 778717505,
            2131414834, 690273672, 1633129278, 275458335, 2948558713, 3736213388, 1165582721,
            619527248, 3580718374, 2331959084, 4015892607, 22218657, 57793748, 78895829,
            2261931181, 2176427545, 254353123, 288354796, 1811254416, 4175221279, 1999304075,
            2243073703, 2406263731, 3607071103, 2509308494, 4227058988, 459041177, 1288003499,
            171257138, 1527074103, 946436740, 4096275409, 3439222084, 1039960427, 682450971,
            624577877, 2383738920, 3781585168, 1998082771, 456216047, 4084776014, 2465435506,
            1212194082, 2219178103, 3115504881, 782294840, 3975566931, 865020752, 3621258027,
            19776526, 3786071932, 3553320495, 1278472279, 360299885, 101923276, 30803414,
            705340594, 3798470694, 1725093175, 3767628802, 2745865016, 2628443054, 260876871,
            511035750, 4013822465, 52914465, 1896658602, 1346680681, 2022488278, 1329922307,
            3852700935, 834456360, 1442297319, 842245334,
        ],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_low_same_length_divide_and_conquer_shared_scratch_fail_1() {
    let mut out = vec![10, 10, 10, 10, 10];
    limbs_mul_low_same_length_divide_and_conquer_shared_scratch(&mut out, &[6, 7], &[1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_low_same_length_divide_and_conquer_shared_scratch_fail_2() {
    let mut out = vec![10];
    limbs_mul_low_same_length_divide_and_conquer_shared_scratch(&mut out, &[6, 7], &[1, 2]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_low_same_length_divide_and_conquer_shared_scratch_fail_3() {
    let mut out = vec![10];
    limbs_mul_low_same_length_divide_and_conquer_shared_scratch(&mut out, &[2], &[3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_low_same_length_divide_and_conquer_fail_1() {
    let mut out = vec![10, 10, 10, 10, 10];
    let mut scratch = vec![0; 6];
    limbs_mul_low_same_length_divide_and_conquer(&mut out, &[6, 7], &[1, 2, 3], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_low_same_length_divide_and_conquer_fail_2() {
    let mut out = vec![10];
    let mut scratch = vec![0; 4];
    limbs_mul_low_same_length_divide_and_conquer(&mut out, &[6, 7], &[1, 2], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_low_same_length_divide_and_conquer_fail_3() {
    let mut out = vec![10];
    let mut scratch = vec![0; 6];
    limbs_mul_low_same_length_divide_and_conquer(&mut out, &[2], &[3], &mut scratch);
}

#[test]
fn limbs_mul_low_same_length_divide_and_conquer_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 512);
    config.insert("mean_stripe_n", 256 << Limb::LOG_WIDTH);
    unsigned_vec_triple_gen_var_25().test_properties_with_config(
        &config,
        |(out_before, xs, ys)| {
            let mut out = out_before.to_vec();
            limbs_mul_low_same_length_divide_and_conquer_shared_scratch(&mut out, &xs, &ys);

            let len = xs.len();
            let out_after = out[..len].to_vec();
            let mut out = out_before.to_vec();
            let mut scratch = vec![0; xs.len() << 1];
            limbs_mul_low_same_length_divide_and_conquer(&mut out, &xs, &ys, &mut scratch);
            let out_after: &[Limb] = &out_after;
            assert_eq!(&out[..len], out_after);

            verify_mul_low_2(&xs, &ys, out_after);
        },
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mul_low_same_length() {
    let test = |xs: Vec<Limb>, ys: Vec<Limb>, out_before: Vec<Limb>, out_after: &[Limb]| {
        let len = xs.len();
        let mut out = out_before.clone();
        limbs_mul_low_same_length(&mut out, &xs, &ys);
        assert_eq!(&out[..len], out_after);
        verify_mul_low_1(&out_before, &xs, &ys, &out);
    };
    // - MULLO_BASECASE_THRESHOLD <= n < MULLO_DC_THRESHOLD
    test(vec![1; 3], series(1, 3), vec![5; 8], &[1, 3, 6]);
    test(
        vec![100, 101, 102],
        vec![102, 101, 100],
        vec![10; 7],
        &[10200, 20402, 30605],
    );
    test(
        vec![u32::MAX; 3],
        vec![u32::MAX; 3],
        vec![10; 6],
        &[1, 0, 0],
    );
    // - n >= MULLO_DC_THRESHOLD
    // - n < MULLO_MUL_N_THRESHOLD
    test(
        vec![
            667555911, 2348733364, 1961106649, 1440628769, 1419633260, 3052369574, 1108620071,
            3434026162, 3916636599, 3102975331, 3886608438, 726540295, 1959301605, 1097548123,
            4197775113, 2426454473, 1977350598, 815012862, 2487421470, 2968184113, 3186369366,
            2845700438, 1474463355, 3590404890, 2351730037, 3978816218, 227579243, 185502596,
            1922772057, 2864373880, 1909608765, 307719594, 4182459185, 3812324913, 1740357086,
            619281590, 1426834026, 2868540501, 440166317, 3324081248, 1368857307, 3133154844,
            1142185935, 2703145826, 1436656515, 2490167985, 2881509383, 725592411, 1056415214,
            1801603748, 1098036334, 728276877, 1517386665, 1881520126, 2784785117, 2287558410,
            3556820397, 1380321205, 706755221, 2829962299, 3613994343, 1462484606, 3627636556,
            490302213, 2592459816, 866144376, 609122783, 1451416993, 3785904246, 4131235963,
            74121435, 3878638497, 588893998, 1092662611, 3469958113, 2363382228, 2678844074,
            1733088933, 3784272536, 4005990872, 2863454468, 3205477346, 3748950603, 3944338479,
            59852399, 3489893816, 1468407650, 596065110, 1335233230, 3643254705, 1408578383,
            2465303822, 2349399082, 3471899735, 2696915607, 1269986424,
        ],
        vec![
            2722576082, 2281236300, 3094793404, 2870225073, 1804040671, 2878250398, 737079723,
            822732050, 655707872, 4207992704, 1851690693, 912905035, 3291778825, 2516279380,
            636556658, 2839780581, 3193297014, 1756749995, 3651621870, 628948913, 380312917,
            2361120672, 2262818273, 2071766395, 4162768312, 2516781023, 3406285004, 1455245572,
            2587465945, 1378482824, 241323934, 2280756750, 242112740, 69419369, 2603755088,
            2163412563, 1341439609, 321882770, 736087982, 2995521870, 3671192545, 1265948417,
            548486283, 3124707078, 290930553, 742853646, 118648394, 3811259549, 2371785381,
            2042286901, 3723558867, 783245266, 2393779385, 1940230299, 1556220091, 3235087403,
            2441469134, 1125637818, 1712201794, 2216073164, 4175845099, 958349548, 4262398424,
            3171987471, 363107024, 2664611701, 2971536098, 2823614641, 1683011498, 1406296445,
            3951206397, 3996972934, 3905528336, 920273699, 1258344157, 2971218980, 1721322990,
            1804720416, 3946193389, 1866895548, 2875334355, 2152965895, 1192611565, 3662025315,
            3450196924, 3273347968, 3209563794, 1516062635, 679683317, 1998597190, 2857249714,
            771410307, 3851158594, 2246899647, 1910389835, 3917001975,
        ],
        vec![10; 96],
        &[
            4002931774, 1904598186, 3234829245, 3606564796, 1306091105, 2193187486, 3152048235,
            591394468, 3956728348, 1387100164, 1906336788, 3282287701, 520335280, 185717337,
            2805648626, 3053703744, 140292430, 3997128531, 231327311, 15232560, 1661811744,
            661469371, 576722126, 854577401, 4168087387, 3545117380, 2423373377, 3239419618,
            806391738, 3880336495, 622341226, 2747181685, 1811142986, 3822894835, 1547377963,
            661471733, 100990771, 311111243, 617447537, 2129281438, 830310054, 2479129328,
            3642060387, 3489744434, 1954771548, 3535568842, 3876438817, 943110050, 2741340933,
            1863625282, 3029160300, 687011149, 2856703096, 1137282724, 1230235236, 3485141654,
            2955294074, 4088046039, 1413613081, 4175441078, 3746030422, 287749227, 1646738360,
            1501205414, 1096064838, 1755515350, 2556528506, 238519283, 3569605126, 796573795,
            4090746106, 4149522856, 583205784, 2821201119, 3555712047, 1090519982, 2976948004,
            3580487377, 627104596, 1130079835, 490101583, 1570134181, 648065435, 1085439524,
            3896882529, 2919299575, 3301848876, 1413509397, 3259269725, 2509252448, 2214096765,
            1288915975, 2839044440, 1348673262, 916617232, 3060106377,
        ],
    );
    // - n >= MULLO_MUL_N_THRESHOLD
    // - !TUNE_PROGRAM_BUILD && MULLO_MUL_N_THRESHOLD > MUL_FFT_THRESHOLD
    test(
        series(1, 9000),
        series(2, 9000),
        vec![10; 9000],
        &[
            2, 7, 16, 30, 50, 77, 112, 156, 210, 275, 352, 442, 546, 665, 800, 952, 1122, 1311,
            1520, 1750, 2002, 2277, 2576, 2900, 3250, 3627, 4032, 4466, 4930, 5425, 5952, 6512,
            7106, 7735, 8400, 9102, 9842, 10621, 11440, 12300, 13202, 14147, 15136, 16170, 17250,
            18377, 19552, 20776, 22050, 23375, 24752, 26182, 27666, 29205, 30800, 32452, 34162,
            35931, 37760, 39650, 41602, 43617, 45696, 47840, 50050, 52327, 54672, 57086, 59570,
            62125, 64752, 67452, 70226, 73075, 76000, 79002, 82082, 85241, 88480, 91800, 95202,
            98687, 102256, 105910, 109650, 113477, 117392, 121396, 125490, 129675, 133952, 138322,
            142786, 147345, 152000, 156752, 161602, 166551, 171600, 176750, 182002, 187357, 192816,
            198380, 204050, 209827, 215712, 221706, 227810, 234025, 240352, 246792, 253346, 260015,
            266800, 273702, 280722, 287861, 295120, 302500, 310002, 317627, 325376, 333250, 341250,
            349377, 357632, 366016, 374530, 383175, 391952, 400862, 409906, 419085, 428400, 437852,
            447442, 457171, 467040, 477050, 487202, 497497, 507936, 518520, 529250, 540127, 551152,
            562326, 573650, 585125, 596752, 608532, 620466, 632555, 644800, 657202, 669762, 682481,
            695360, 708400, 721602, 734967, 748496, 762190, 776050, 790077, 804272, 818636, 833170,
            847875, 862752, 877802, 893026, 908425, 924000, 939752, 955682, 971791, 988080,
            1004550, 1021202, 1038037, 1055056, 1072260, 1089650, 1107227, 1124992, 1142946,
            1161090, 1179425, 1197952, 1216672, 1235586, 1254695, 1274000, 1293502, 1313202,
            1333101, 1353200, 1373500, 1394002, 1414707, 1435616, 1456730, 1478050, 1499577,
            1521312, 1543256, 1565410, 1587775, 1610352, 1633142, 1656146, 1679365, 1702800,
            1726452, 1750322, 1774411, 1798720, 1823250, 1848002, 1872977, 1898176, 1923600,
            1949250, 1975127, 2001232, 2027566, 2054130, 2080925, 2107952, 2135212, 2162706,
            2190435, 2218400, 2246602, 2275042, 2303721, 2332640, 2361800, 2391202, 2420847,
            2450736, 2480870, 2511250, 2541877, 2572752, 2603876, 2635250, 2666875, 2698752,
            2730882, 2763266, 2795905, 2828800, 2861952, 2895362, 2929031, 2962960, 2997150,
            3031602, 3066317, 3101296, 3136540, 3172050, 3207827, 3243872, 3280186, 3316770,
            3353625, 3390752, 3428152, 3465826, 3503775, 3542000, 3580502, 3619282, 3658341,
            3697680, 3737300, 3777202, 3817387, 3857856, 3898610, 3939650, 3980977, 4022592,
            4064496, 4106690, 4149175, 4191952, 4235022, 4278386, 4322045, 4366000, 4410252,
            4454802, 4499651, 4544800, 4590250, 4636002, 4682057, 4728416, 4775080, 4822050,
            4869327, 4916912, 4964806, 5013010, 5061525, 5110352, 5159492, 5208946, 5258715,
            5308800, 5359202, 5409922, 5460961, 5512320, 5564000, 5616002, 5668327, 5720976,
            5773950, 5827250, 5880877, 5934832, 5989116, 6043730, 6098675, 6153952, 6209562,
            6265506, 6321785, 6378400, 6435352, 6492642, 6550271, 6608240, 6666550, 6725202,
            6784197, 6843536, 6903220, 6963250, 7023627, 7084352, 7145426, 7206850, 7268625,
            7330752, 7393232, 7456066, 7519255, 7582800, 7646702, 7710962, 7775581, 7840560,
            7905900, 7971602, 8037667, 8104096, 8170890, 8238050, 8305577, 8373472, 8441736,
            8510370, 8579375, 8648752, 8718502, 8788626, 8859125, 8930000, 9001252, 9072882,
            9144891, 9217280, 9290050, 9363202, 9436737, 9510656, 9584960, 9659650, 9734727,
            9810192, 9886046, 9962290, 10038925, 10115952, 10193372, 10271186, 10349395, 10428000,
            10507002, 10586402, 10666201, 10746400, 10827000, 10908002, 10989407, 11071216,
            11153430, 11236050, 11319077, 11402512, 11486356, 11570610, 11655275, 11740352,
            11825842, 11911746, 11998065, 12084800, 12171952, 12259522, 12347511, 12435920,
            12524750, 12614002, 12703677, 12793776, 12884300, 12975250, 13066627, 13158432,
            13250666, 13343330, 13436425, 13529952, 13623912, 13718306, 13813135, 13908400,
            14004102, 14100242, 14196821, 14293840, 14391300, 14489202, 14587547, 14686336,
            14785570, 14885250, 14985377, 15085952, 15186976, 15288450, 15390375, 15492752,
            15595582, 15698866, 15802605, 15906800, 16011452, 16116562, 16222131, 16328160,
            16434650, 16541602, 16649017, 16756896, 16865240, 16974050, 17083327, 17193072,
            17303286, 17413970, 17525125, 17636752, 17748852, 17861426, 17974475, 18088000,
            18202002, 18316482, 18431441, 18546880, 18662800, 18779202, 18896087, 19013456,
            19131310, 19249650, 19368477, 19487792, 19607596, 19727890, 19848675, 19969952,
            20091722, 20213986, 20336745, 20460000, 20583752, 20708002, 20832751, 20958000,
            21083750, 21210002, 21336757, 21464016, 21591780, 21720050, 21848827, 21978112,
            22107906, 22238210, 22369025, 22500352, 22632192, 22764546, 22897415, 23030800,
            23164702, 23299122, 23434061, 23569520, 23705500, 23842002, 23979027, 24116576,
            24254650, 24393250, 24532377, 24672032, 24812216, 24952930, 25094175, 25235952,
            25378262, 25521106, 25664485, 25808400, 25952852, 26097842, 26243371, 26389440,
            26536050, 26683202, 26830897, 26979136, 27127920, 27277250, 27427127, 27577552,
            27728526, 27880050, 28032125, 28184752, 28337932, 28491666, 28645955, 28800800,
            28956202, 29112162, 29268681, 29425760, 29583400, 29741602, 29900367, 30059696,
            30219590, 30380050, 30541077, 30702672, 30864836, 31027570, 31190875, 31354752,
            31519202, 31684226, 31849825, 32016000, 32182752, 32350082, 32517991, 32686480,
            32855550, 33025202, 33195437, 33366256, 33537660, 33709650, 33882227, 34055392,
            34229146, 34403490, 34578425, 34753952, 34930072, 35106786, 35284095, 35462000,
            35640502, 35819602, 35999301, 36179600, 36360500, 36542002, 36724107, 36906816,
            37090130, 37274050, 37458577, 37643712, 37829456, 38015810, 38202775, 38390352,
            38578542, 38767346, 38956765, 39146800, 39337452, 39528722, 39720611, 39913120,
            40106250, 40300002, 40494377, 40689376, 40885000, 41081250, 41278127, 41475632,
            41673766, 41872530, 42071925, 42271952, 42472612, 42673906, 42875835, 43078400,
            43281602, 43485442, 43689921, 43895040, 44100800, 44307202, 44514247, 44721936,
            44930270, 45139250, 45348877, 45559152, 45770076, 45981650, 46193875, 46406752,
            46620282, 46834466, 47049305, 47264800, 47480952, 47697762, 47915231, 48133360,
            48352150, 48571602, 48791717, 49012496, 49233940, 49456050, 49678827, 49902272,
            50126386, 50351170, 50576625, 50802752, 51029552, 51257026, 51485175, 51714000,
            51943502, 52173682, 52404541, 52636080, 52868300, 53101202, 53334787, 53569056,
            53804010, 54039650, 54275977, 54512992, 54750696, 54989090, 55228175, 55467952,
            55708422, 55949586, 56191445, 56434000, 56677252, 56921202, 57165851, 57411200,
            57657250, 57904002, 58151457, 58399616, 58648480, 58898050, 59148327, 59399312,
            59651006, 59903410, 60156525, 60410352, 60664892, 60920146, 61176115, 61432800,
            61690202, 61948322, 62207161, 62466720, 62727000, 62988002, 63249727, 63512176,
            63775350, 64039250, 64303877, 64569232, 64835316, 65102130, 65369675, 65637952,
            65906962, 66176706, 66447185, 66718400, 66990352, 67263042, 67536471, 67810640,
            68085550, 68361202, 68637597, 68914736, 69192620, 69471250, 69750627, 70030752,
            70311626, 70593250, 70875625, 71158752, 71442632, 71727266, 72012655, 72298800,
            72585702, 72873362, 73161781, 73450960, 73740900, 74031602, 74323067, 74615296,
            74908290, 75202050, 75496577, 75791872, 76087936, 76384770, 76682375, 76980752,
            77279902, 77579826, 77880525, 78182000, 78484252, 78787282, 79091091, 79395680,
            79701050, 80007202, 80314137, 80621856, 80930360, 81239650, 81549727, 81860592,
            82172246, 82484690, 82797925, 83111952, 83426772, 83742386, 84058795, 84376000,
            84694002, 85012802, 85332401, 85652800, 85974000, 86296002, 86618807, 86942416,
            87266830, 87592050, 87918077, 88244912, 88572556, 88901010, 89230275, 89560352,
            89891242, 90222946, 90555465, 90888800, 91222952, 91557922, 91893711, 92230320,
            92567750, 92906002, 93245077, 93584976, 93925700, 94267250, 94609627, 94952832,
            95296866, 95641730, 95987425, 96333952, 96681312, 97029506, 97378535, 97728400,
            98079102, 98430642, 98783021, 99136240, 99490300, 99845202, 100200947, 100557536,
            100914970, 101273250, 101632377, 101992352, 102353176, 102714850, 103077375, 103440752,
            103804982, 104170066, 104536005, 104902800, 105270452, 105638962, 106008331, 106378560,
            106749650, 107121602, 107494417, 107868096, 108242640, 108618050, 108994327, 109371472,
            109749486, 110128370, 110508125, 110888752, 111270252, 111652626, 112035875, 112420000,
            112805002, 113190882, 113577641, 113965280, 114353800, 114743202, 115133487, 115524656,
            115916710, 116309650, 116703477, 117098192, 117493796, 117890290, 118287675, 118685952,
            119085122, 119485186, 119886145, 120288000, 120690752, 121094402, 121498951, 121904400,
            122310750, 122718002, 123126157, 123535216, 123945180, 124356050, 124767827, 125180512,
            125594106, 126008610, 126424025, 126840352, 127257592, 127675746, 128094815, 128514800,
            128935702, 129357522, 129780261, 130203920, 130628500, 131054002, 131480427, 131907776,
            132336050, 132765250, 133195377, 133626432, 134058416, 134491330, 134925175, 135359952,
            135795662, 136232306, 136669885, 137108400, 137547852, 137988242, 138429571, 138871840,
            139315050, 139759202, 140204297, 140650336, 141097320, 141545250, 141994127, 142443952,
            142894726, 143346450, 143799125, 144252752, 144707332, 145162866, 145619355, 146076800,
            146535202, 146994562, 147454881, 147916160, 148378400, 148841602, 149305767, 149770896,
            150236990, 150704050, 151172077, 151641072, 152111036, 152581970, 153053875, 153526752,
            154000602, 154475426, 154951225, 155428000, 155905752, 156384482, 156864191, 157344880,
            157826550, 158309202, 158792837, 159277456, 159763060, 160249650, 160737227, 161225792,
            161715346, 162205890, 162697425, 163189952, 163683472, 164177986, 164673495, 165170000,
            165667502, 166166002, 166665501, 167166000, 167667500, 168170002, 168673507, 169178016,
            169683530, 170190050, 170697577, 171206112, 171715656, 172226210, 172737775, 173250352,
            173763942, 174278546, 174794165, 175310800, 175828452, 176347122, 176866811, 177387520,
            177909250, 178432002, 178955777, 179480576, 180006400, 180533250, 181061127, 181590032,
            182119966, 182650930, 183182925, 183715952, 184250012, 184785106, 185321235, 185858400,
            186396602, 186935842, 187476121, 188017440, 188559800, 189103202, 189647647, 190193136,
            190739670, 191287250, 191835877, 192385552, 192936276, 193488050, 194040875, 194594752,
            195149682, 195705666, 196262705, 196820800, 197379952, 197940162, 198501431, 199063760,
            199627150, 200191602, 200757117, 201323696, 201891340, 202460050, 203029827, 203600672,
            204172586, 204745570, 205319625, 205894752, 206470952, 207048226, 207626575, 208206000,
            208786502, 209368082, 209950741, 210534480, 211119300, 211705202, 212292187, 212880256,
            213469410, 214059650, 214650977, 215243392, 215836896, 216431490, 217027175, 217623952,
            218221822, 218820786, 219420845, 220022000, 220624252, 221227602, 221832051, 222437600,
            223044250, 223652002, 224260857, 224870816, 225481880, 226094050, 226707327, 227321712,
            227937206, 228553810, 229171525, 229790352, 230410292, 231031346, 231653515, 232276800,
            232901202, 233526722, 234153361, 234781120, 235410000, 236040002, 236671127, 237303376,
            237936750, 238571250, 239206877, 239843632, 240481516, 241120530, 241760675, 242401952,
            243044362, 243687906, 244332585, 244978400, 245625352, 246273442, 246922671, 247573040,
            248224550, 248877202, 249530997, 250185936, 250842020, 251499250, 252157627, 252817152,
            253477826, 254139650, 254802625, 255466752, 256132032, 256798466, 257466055, 258134800,
            258804702, 259475762, 260147981, 260821360, 261495900, 262171602, 262848467, 263526496,
            264205690, 264886050, 265567577, 266250272, 266934136, 267619170, 268305375, 268992752,
            269681302, 270371026, 271061925, 271754000, 272447252, 273141682, 273837291, 274534080,
            275232050, 275931202, 276631537, 277333056, 278035760, 278739650, 279444727, 280150992,
            280858446, 281567090, 282276925, 282987952, 283700172, 284413586, 285128195, 285844000,
            286561002, 287279202, 287998601, 288719200, 289441000, 290164002, 290888207, 291613616,
            292340230, 293068050, 293797077, 294527312, 295258756, 295991410, 296725275, 297460352,
            298196642, 298934146, 299672865, 300412800, 301153952, 301896322, 302639911, 303384720,
            304130750, 304878002, 305626477, 306376176, 307127100, 307879250, 308632627, 309387232,
            310143066, 310900130, 311658425, 312417952, 313178712, 313940706, 314703935, 315468400,
            316234102, 317001042, 317769221, 318538640, 319309300, 320081202, 320854347, 321628736,
            322404370, 323181250, 323959377, 324738752, 325519376, 326301250, 327084375, 327868752,
            328654382, 329441266, 330229405, 331018800, 331809452, 332601362, 333394531, 334188960,
            334984650, 335781602, 336579817, 337379296, 338180040, 338982050, 339785327, 340589872,
            341395686, 342202770, 343011125, 343820752, 344631652, 345443826, 346257275, 347072000,
            347888002, 348705282, 349523841, 350343680, 351164800, 351987202, 352810887, 353635856,
            354462110, 355289650, 356118477, 356948592, 357779996, 358612690, 359446675, 360281952,
            361118522, 361956386, 362795545, 363636000, 364477752, 365320802, 366165151, 367010800,
            367857750, 368706002, 369555557, 370406416, 371258580, 372112050, 372966827, 373822912,
            374680306, 375539010, 376399025, 377260352, 378122992, 378986946, 379852215, 380718800,
            381586702, 382455922, 383326461, 384198320, 385071500, 385946002, 386821827, 387698976,
            388577450, 389457250, 390338377, 391220832, 392104616, 392989730, 393876175, 394763952,
            395653062, 396543506, 397435285, 398328400, 399222852, 400118642, 401015771, 401914240,
            402814050, 403715202, 404617697, 405521536, 406426720, 407333250, 408241127, 409150352,
            410060926, 410972850, 411886125, 412800752, 413716732, 414634066, 415552755, 416472800,
            417394202, 418316962, 419241081, 420166560, 421093400, 422021602, 422951167, 423882096,
            424814390, 425748050, 426683077, 427619472, 428557236, 429496370, 430436875, 431378752,
            432322002, 433266626, 434212625, 435160000, 436108752, 437058882, 438010391, 438963280,
            439917550, 440873202, 441830237, 442788656, 443748460, 444709650, 445672227, 446636192,
            447601546, 448568290, 449536425, 450505952, 451476872, 452449186, 453422895, 454398000,
            455374502, 456352402, 457331701, 458312400, 459294500, 460278002, 461262907, 462249216,
            463236930, 464226050, 465216577, 466208512, 467201856, 468196610, 469192775, 470190352,
            471189342, 472189746, 473191565, 474194800, 475199452, 476205522, 477213011, 478221920,
            479232250, 480244002, 481257177, 482271776, 483287800, 484305250, 485324127, 486344432,
            487366166, 488389330, 489413925, 490439952, 491467412, 492496306, 493526635, 494558400,
            495591602, 496626242, 497662321, 498699840, 499738800, 500779202, 501821047, 502864336,
            503909070, 504955250, 506002877, 507051952, 508102476, 509154450, 510207875, 511262752,
            512319082, 513376866, 514436105, 515496800, 516558952, 517622562, 518687631, 519754160,
            520822150, 521891602, 522962517, 524034896, 525108740, 526184050, 527260827, 528339072,
            529418786, 530499970, 531582625, 532666752, 533752352, 534839426, 535927975, 537018000,
            538109502, 539202482, 540296941, 541392880, 542490300, 543589202, 544689587, 545791456,
            546894810, 547999650, 549105977, 550213792, 551323096, 552433890, 553546175, 554659952,
            555775222, 556891986, 558010245, 559130000, 560251252, 561374002, 562498251, 563624000,
            564751250, 565880002, 567010257, 568142016, 569275280, 570410050, 571546327, 572684112,
            573823406, 574964210, 576106525, 577250352, 578395692, 579542546, 580690915, 581840800,
            582992202, 584145122, 585299561, 586455520, 587613000, 588772002, 589932527, 591094576,
            592258150, 593423250, 594589877, 595758032, 596927716, 598098930, 599271675, 600445952,
            601621762, 602799106, 603977985, 605158400, 606340352, 607523842, 608708871, 609895440,
            611083550, 612273202, 613464397, 614657136, 615851420, 617047250, 618244627, 619443552,
            620644026, 621846050, 623049625, 624254752, 625461432, 626669666, 627879455, 629090800,
            630303702, 631518162, 632734181, 633951760, 635170900, 636391602, 637613867, 638837696,
            640063090, 641290050, 642518577, 643748672, 644980336, 646213570, 647448375, 648684752,
            649922702, 651162226, 652403325, 653646000, 654890252, 656136082, 657383491, 658632480,
            659883050, 661135202, 662388937, 663644256, 664901160, 666159650, 667419727, 668681392,
            669944646, 671209490, 672475925, 673743952, 675013572, 676284786, 677557595, 678832000,
            680108002, 681385602, 682664801, 683945600, 685228000, 686512002, 687797607, 689084816,
            690373630, 691664050, 692956077, 694249712, 695544956, 696841810, 698140275, 699440352,
            700742042, 702045346, 703350265, 704656800, 705964952, 707274722, 708586111, 709899120,
            711213750, 712530002, 713847877, 715167376, 716488500, 717811250, 719135627, 720461632,
            721789266, 723118530, 724449425, 725781952, 727116112, 728451906, 729789335, 731128400,
            732469102, 733811442, 735155421, 736501040, 737848300, 739197202, 740547747, 741899936,
            743253770, 744609250, 745966377, 747325152, 748685576, 750047650, 751411375, 752776752,
            754143782, 755512466, 756882805, 758254800, 759628452, 761003762, 762380731, 763759360,
            765139650, 766521602, 767905217, 769290496, 770677440, 772066050, 773456327, 774848272,
            776241886, 777637170, 779034125, 780432752, 781833052, 783235026, 784638675, 786044000,
            787451002, 788859682, 790270041, 791682080, 793095800, 794511202, 795928287, 797347056,
            798767510, 800189650, 801613477, 803038992, 804466196, 805895090, 807325675, 808757952,
            810191922, 811627586, 813064945, 814504000, 815944752, 817387202, 818831351, 820277200,
            821724750, 823174002, 824624957, 826077616, 827531980, 828988050, 830445827, 831905312,
            833366506, 834829410, 836294025, 837760352, 839228392, 840698146, 842169615, 843642800,
            845117702, 846594322, 848072661, 849552720, 851034500, 852518002, 854003227, 855490176,
            856978850, 858469250, 859961377, 861455232, 862950816, 864448130, 865947175, 867447952,
            868950462, 870454706, 871960685, 873468400, 874977852, 876489042, 878001971, 879516640,
            881033050, 882551202, 884071097, 885592736, 887116120, 888641250, 890168127, 891696752,
            893227126, 894759250, 896293125, 897828752, 899366132, 900905266, 902446155, 903988800,
            905533202, 907079362, 908627281, 910176960, 911728400, 913281602, 914836567, 916393296,
            917951790, 919512050, 921074077, 922637872, 924203436, 925770770, 927339875, 928910752,
            930483402, 932057826, 933634025, 935212000, 936791752, 938373282, 939956591, 941541680,
            943128550, 944717202, 946307637, 947899856, 949493860, 951089650, 952687227, 954286592,
            955887746, 957490690, 959095425, 960701952, 962310272, 963920386, 965532295, 967146000,
            968761502, 970378802, 971997901, 973618800, 975241500, 976866002, 978492307, 980120416,
            981750330, 983382050, 985015577, 986650912, 988288056, 989927010, 991567775, 993210352,
            994854742, 996500946, 998148965, 999798800, 1001450452, 1003103922, 1004759211,
            1006416320, 1008075250, 1009736002, 1011398577, 1013062976, 1014729200, 1016397250,
            1018067127, 1019738832, 1021412366, 1023087730, 1024764925, 1026443952, 1028124812,
            1029807506, 1031492035, 1033178400, 1034866602, 1036556642, 1038248521, 1039942240,
            1041637800, 1043335202, 1045034447, 1046735536, 1048438470, 1050143250, 1051849877,
            1053558352, 1055268676, 1056980850, 1058694875, 1060410752, 1062128482, 1063848066,
            1065569505, 1067292800, 1069017952, 1070744962, 1072473831, 1074204560, 1075937150,
            1077671602, 1079407917, 1081146096, 1082886140, 1084628050, 1086371827, 1088117472,
            1089864986, 1091614370, 1093365625, 1095118752, 1096873752, 1098630626, 1100389375,
            1102150000, 1103912502, 1105676882, 1107443141, 1109211280, 1110981300, 1112753202,
            1114526987, 1116302656, 1118080210, 1119859650, 1121640977, 1123424192, 1125209296,
            1126996290, 1128785175, 1130575952, 1132368622, 1134163186, 1135959645, 1137758000,
            1139558252, 1141360402, 1143164451, 1144970400, 1146778250, 1148588002, 1150399657,
            1152213216, 1154028680, 1155846050, 1157665327, 1159486512, 1161309606, 1163134610,
            1164961525, 1166790352, 1168621092, 1170453746, 1172288315, 1174124800, 1175963202,
            1177803522, 1179645761, 1181489920, 1183336000, 1185184002, 1187033927, 1188885776,
            1190739550, 1192595250, 1194452877, 1196312432, 1198173916, 1200037330, 1201902675,
            1203769952, 1205639162, 1207510306, 1209383385, 1211258400, 1213135352, 1215014242,
            1216895071, 1218777840, 1220662550, 1222549202, 1224437797, 1226328336, 1228220820,
            1230115250, 1232011627, 1233909952, 1235810226, 1237712450, 1239616625, 1241522752,
            1243430832, 1245340866, 1247252855, 1249166800, 1251082702, 1253000562, 1254920381,
            1256842160, 1258765900, 1260691602, 1262619267, 1264548896, 1266480490, 1268414050,
            1270349577, 1272287072, 1274226536, 1276167970, 1278111375, 1280056752, 1282004102,
            1283953426, 1285904725, 1287858000, 1289813252, 1291770482, 1293729691, 1295690880,
            1297654050, 1299619202, 1301586337, 1303555456, 1305526560, 1307499650, 1309474727,
            1311451792, 1313430846, 1315411890, 1317394925, 1319379952, 1321366972, 1323355986,
            1325346995, 1327340000, 1329335002, 1331332002, 1333331001, 1335332000, 1337335000,
            1339340002, 1341347007, 1343356016, 1345367030, 1347380050, 1349395077, 1351412112,
            1353431156, 1355452210, 1357475275, 1359500352, 1361527442, 1363556546, 1365587665,
            1367620800, 1369655952, 1371693122, 1373732311, 1375773520, 1377816750, 1379862002,
            1381909277, 1383958576, 1386009900, 1388063250, 1390118627, 1392176032, 1394235466,
            1396296930, 1398360425, 1400425952, 1402493512, 1404563106, 1406634735, 1408708400,
            1410784102, 1412861842, 1414941621, 1417023440, 1419107300, 1421193202, 1423281147,
            1425371136, 1427463170, 1429557250, 1431653377, 1433751552, 1435851776, 1437954050,
            1440058375, 1442164752, 1444273182, 1446383666, 1448496205, 1450610800, 1452727452,
            1454846162, 1456966931, 1459089760, 1461214650, 1463341602, 1465470617, 1467601696,
            1469734840, 1471870050, 1474007327, 1476146672, 1478288086, 1480431570, 1482577125,
            1484724752, 1486874452, 1489026226, 1491180075, 1493336000, 1495494002, 1497654082,
            1499816241, 1501980480, 1504146800, 1506315202, 1508485687, 1510658256, 1512832910,
            1515009650, 1517188477, 1519369392, 1521552396, 1523737490, 1525924675, 1528113952,
            1530305322, 1532498786, 1534694345, 1536892000, 1539091752, 1541293602, 1543497551,
            1545703600, 1547911750, 1550122002, 1552334357, 1554548816, 1556765380, 1558984050,
            1561204827, 1563427712, 1565652706, 1567879810, 1570109025, 1572340352, 1574573792,
            1576809346, 1579047015, 1581286800, 1583528702, 1585772722, 1588018861, 1590267120,
            1592517500, 1594770002, 1597024627, 1599281376, 1601540250, 1603801250, 1606064377,
            1608329632, 1610597016, 1612866530, 1615138175, 1617411952, 1619687862, 1621965906,
            1624246085, 1626528400, 1628812852, 1631099442, 1633388171, 1635679040, 1637972050,
            1640267202, 1642564497, 1644863936, 1647165520, 1649469250, 1651775127, 1654083152,
            1656393326, 1658705650, 1661020125, 1663336752, 1665655532, 1667976466, 1670299555,
            1672624800, 1674952202, 1677281762, 1679613481, 1681947360, 1684283400, 1686621602,
            1688961967, 1691304496, 1693649190, 1695996050, 1698345077, 1700696272, 1703049636,
            1705405170, 1707762875, 1710122752, 1712484802, 1714849026, 1717215425, 1719584000,
            1721954752, 1724327682, 1726702791, 1729080080, 1731459550, 1733841202, 1736225037,
            1738611056, 1740999260, 1743389650, 1745782227, 1748176992, 1750573946, 1752973090,
            1755374425, 1757777952, 1760183672, 1762591586, 1765001695, 1767414000, 1769828502,
            1772245202, 1774664101, 1777085200, 1779508500, 1781934002, 1784361707, 1786791616,
            1789223730, 1791658050, 1794094577, 1796533312, 1798974256, 1801417410, 1803862775,
            1806310352, 1808760142, 1811212146, 1813666365, 1816122800, 1818581452, 1821042322,
            1823505411, 1825970720, 1828438250, 1830908002, 1833379977, 1835854176, 1838330600,
            1840809250, 1843290127, 1845773232, 1848258566, 1850746130, 1853235925, 1855727952,
            1858222212, 1860718706, 1863217435, 1865718400, 1868221602, 1870727042, 1873234721,
            1875744640, 1878256800, 1880771202, 1883287847, 1885806736, 1888327870, 1890851250,
            1893376877, 1895904752, 1898434876, 1900967250, 1903501875, 1906038752, 1908577882,
            1911119266, 1913662905, 1916208800, 1918756952, 1921307362, 1923860031, 1926414960,
            1928972150, 1931531602, 1934093317, 1936657296, 1939223540, 1941792050, 1944362827,
            1946935872, 1949511186, 1952088770, 1954668625, 1957250752, 1959835152, 1962421826,
            1965010775, 1967602000, 1970195502, 1972791282, 1975389341, 1977989680, 1980592300,
            1983197202, 1985804387, 1988413856, 1991025610, 1993639650, 1996255977, 1998874592,
            2001495496, 2004118690, 2006744175, 2009371952, 2012002022, 2014634386, 2017269045,
            2019906000, 2022545252, 2025186802, 2027830651, 2030476800, 2033125250, 2035776002,
            2038429057, 2041084416, 2043742080, 2046402050, 2049064327, 2051728912, 2054395806,
            2057065010, 2059736525, 2062410352, 2065086492, 2067764946, 2070445715, 2073128800,
            2075814202, 2078501922, 2081191961, 2083884320, 2086579000, 2089276002, 2091975327,
            2094676976, 2097380950, 2100087250, 2102795877, 2105506832, 2108220116, 2110935730,
            2113653675, 2116373952, 2119096562, 2121821506, 2124548785, 2127278400, 2130010352,
            2132744642, 2135481271, 2138220240, 2140961550, 2143705202, 2146451197, 2149199536,
            2151950220, 2154703250, 2157458627, 2160216352, 2162976426, 2165738850, 2168503625,
            2171270752, 2174040232, 2176812066, 2179586255, 2182362800, 2185141702, 2187922962,
            2190706581, 2193492560, 2196280900, 2199071602, 2201864667, 2204660096, 2207457890,
            2210258050, 2213060577, 2215865472, 2218672736, 2221482370, 2224294375, 2227108752,
            2229925502, 2232744626, 2235566125, 2238390000, 2241216252, 2244044882, 2246875891,
            2249709280, 2252545050, 2255383202, 2258223737, 2261066656, 2263911960, 2266759650,
            2269609727, 2272462192, 2275317046, 2278174290, 2281033925, 2283895952, 2286760372,
            2289627186, 2292496395, 2295368000, 2298242002, 2301118402, 2303997201, 2306878400,
            2309762000, 2312648002, 2315536407, 2318427216, 2321320430, 2324216050, 2327114077,
            2330014512, 2332917356, 2335822610, 2338730275, 2341640352, 2344552842, 2347467746,
            2350385065, 2353304800, 2356226952, 2359151522, 2362078511, 2365007920, 2367939750,
            2370874002, 2373810677, 2376749776, 2379691300, 2382635250, 2385581627, 2388530432,
            2391481666, 2394435330, 2397391425, 2400349952, 2403310912, 2406274306, 2409240135,
            2412208400, 2415179102, 2418152242, 2421127821, 2424105840, 2427086300, 2430069202,
            2433054547, 2436042336, 2439032570, 2442025250, 2445020377, 2448017952, 2451017976,
            2454020450, 2457025375, 2460032752, 2463042582, 2466054866, 2469069605, 2472086800,
            2475106452, 2478128562, 2481153131, 2484180160, 2487209650, 2490241602, 2493276017,
            2496312896, 2499352240, 2502394050, 2505438327, 2508485072, 2511534286, 2514585970,
            2517640125, 2520696752, 2523755852, 2526817426, 2529881475, 2532948000, 2536017002,
            2539088482, 2542162441, 2545238880, 2548317800, 2551399202, 2554483087, 2557569456,
            2560658310, 2563749650, 2566843477, 2569939792, 2573038596, 2576139890, 2579243675,
            2582349952, 2585458722, 2588569986, 2591683745, 2594800000, 2597918752, 2601040002,
            2604163751, 2607290000, 2610418750, 2613550002, 2616683757, 2619820016, 2622958780,
            2626100050, 2629243827, 2632390112, 2635538906, 2638690210, 2641844025, 2645000352,
            2648159192, 2651320546, 2654484415, 2657650800, 2660819702, 2663991122, 2667165061,
            2670341520, 2673520500, 2676702002, 2679886027, 2683072576, 2686261650, 2689453250,
            2692647377, 2695844032, 2699043216, 2702244930, 2705449175, 2708655952, 2711865262,
            2715077106, 2718291485, 2721508400, 2724727852, 2727949842, 2731174371, 2734401440,
            2737631050, 2740863202, 2744097897, 2747335136, 2750574920, 2753817250, 2757062127,
            2760309552, 2763559526, 2766812050, 2770067125, 2773324752, 2776584932, 2779847666,
            2783112955, 2786380800, 2789651202, 2792924162, 2796199681, 2799477760, 2802758400,
            2806041602, 2809327367, 2812615696, 2815906590, 2819200050, 2822496077, 2825794672,
            2829095836, 2832399570, 2835705875, 2839014752, 2842326202, 2845640226, 2848956825,
            2852276000, 2855597752, 2858922082, 2862248991, 2865578480, 2868910550, 2872245202,
            2875582437, 2878922256, 2882264660, 2885609650, 2888957227, 2892307392, 2895660146,
            2899015490, 2902373425, 2905733952, 2909097072, 2912462786, 2915831095, 2919202000,
            2922575502, 2925951602, 2929330301, 2932711600, 2936095500, 2939482002, 2942871107,
            2946262816, 2949657130, 2953054050, 2956453577, 2959855712, 2963260456, 2966667810,
            2970077775, 2973490352, 2976905542, 2980323346, 2983743765, 2987166800, 2990592452,
            2994020722, 2997451611, 3000885120, 3004321250, 3007760002, 3011201377, 3014645376,
            3018092000, 3021541250, 3024993127, 3028447632, 3031904766, 3035364530, 3038826925,
            3042291952, 3045759612, 3049229906, 3052702835, 3056178400, 3059656602, 3063137442,
            3066620921, 3070107040, 3073595800, 3077087202, 3080581247, 3084077936, 3087577270,
            3091079250, 3094583877, 3098091152, 3101601076, 3105113650, 3108628875, 3112146752,
            3115667282, 3119190466, 3122716305, 3126244800, 3129775952, 3133309762, 3136846231,
            3140385360, 3143927150, 3147471602, 3151018717, 3154568496, 3158120940, 3161676050,
            3165233827, 3168794272, 3172357386, 3175923170, 3179491625, 3183062752, 3186636552,
            3190213026, 3193792175, 3197374000, 3200958502, 3204545682, 3208135541, 3211728080,
            3215323300, 3218921202, 3222521787, 3226125056, 3229731010, 3233339650, 3236950977,
            3240564992, 3244181696, 3247801090, 3251423175, 3255047952, 3258675422, 3262305586,
            3265938445, 3269574000, 3273212252, 3276853202, 3280496851, 3284143200, 3287792250,
            3291444002, 3295098457, 3298755616, 3302415480, 3306078050, 3309743327, 3313411312,
            3317082006, 3320755410, 3324431525, 3328110352, 3331791892, 3335476146, 3339163115,
            3342852800, 3346545202, 3350240322, 3353938161, 3357638720, 3361342000, 3365048002,
            3368756727, 3372468176, 3376182350, 3379899250, 3383618877, 3387341232, 3391066316,
            3394794130, 3398524675, 3402257952, 3405993962, 3409732706, 3413474185, 3417218400,
            3420965352, 3424715042, 3428467471, 3432222640, 3435980550, 3439741202, 3443504597,
            3447270736, 3451039620, 3454811250, 3458585627, 3462362752, 3466142626, 3469925250,
            3473710625, 3477498752, 3481289632, 3485083266, 3488879655, 3492678800, 3496480702,
            3500285362, 3504092781, 3507902960, 3511715900, 3515531602, 3519350067, 3523171296,
            3526995290, 3530822050, 3534651577, 3538483872, 3542318936, 3546156770, 3549997375,
            3553840752, 3557686902, 3561535826, 3565387525, 3569242000, 3573099252, 3576959282,
            3580822091, 3584687680, 3588556050, 3592427202, 3596301137, 3600177856, 3604057360,
            3607939650, 3611824727, 3615712592, 3619603246, 3623496690, 3627392925, 3631291952,
            3635193772, 3639098386, 3643005795, 3646916000, 3650829002, 3654744802, 3658663401,
            3662584800, 3666509000, 3670436002, 3674365807, 3678298416, 3682233830, 3686172050,
            3690113077, 3694056912, 3698003556, 3701953010, 3705905275, 3709860352, 3713818242,
            3717778946, 3721742465, 3725708800, 3729677952, 3733649922, 3737624711, 3741602320,
            3745582750, 3749566002, 3753552077, 3757540976, 3761532700, 3765527250, 3769524627,
            3773524832, 3777527866, 3781533730, 3785542425, 3789553952, 3793568312, 3797585506,
            3801605535, 3805628400, 3809654102, 3813682642, 3817714021, 3821748240, 3825785300,
            3829825202, 3833867947, 3837913536, 3841961970, 3846013250, 3850067377, 3854124352,
            3858184176, 3862246850, 3866312375, 3870380752, 3874451982, 3878526066, 3882603005,
            3886682800, 3890765452, 3894850962, 3898939331, 3903030560, 3907124650, 3911221602,
            3915321417, 3919424096, 3923529640, 3927638050, 3931749327, 3935863472, 3939980486,
            3944100370, 3948223125, 3952348752, 3956477252, 3960608626, 3964742875, 3968880000,
            3973020002, 3977162882, 3981308641, 3985457280, 3989608800, 3993763202, 3997920487,
            4002080656, 4006243710, 4010409650, 4014578477, 4018750192, 4022924796, 4027102290,
            4031282675, 4035465952, 4039652122, 4043841186, 4048033145, 4052228000, 4056425752,
            4060626402, 4064829951, 4069036400, 4073245750, 4077458002, 4081673157, 4085891216,
            4090112180, 4094336050, 4098562827, 4102792512, 4107025106, 4111260610, 4115499025,
            4119740352, 4123984592, 4128231746, 4132481815, 4136734800, 4140990702, 4145249522,
            4149511261, 4153775920, 4158043500, 4162314002, 4166587427, 4170863776, 4175143050,
            4179425250, 4183710377, 4187998432, 4192289416, 4196583330, 4200880175, 4205179952,
            4209482662, 4213788306, 4218096885, 4222408400, 4226722852, 4231040242, 4235360571,
            4239683840, 4244010050, 4248339202, 4252671297, 4257006336, 4261344320, 4265685250,
            4270029127, 4274375952, 4278725726, 4283078450, 4287434125, 4291792752, 1187036,
            5551571, 9919060, 14289505, 18662907, 23039267, 27418586, 31800865, 36186105, 40574307,
            44965472, 49359601, 53756695, 58156755, 62559782, 66965777, 71374741, 75786675,
            80201580, 84619457, 89040307, 93464131, 97890930, 102320705, 106753457, 111189187,
            115627896, 120069585, 124514255, 128961907, 133412542, 137866161, 142322765, 146782355,
            151244932, 155710497, 160179051, 164650595, 169125130, 173602657, 178083177, 182566691,
            187053200, 191542705, 196035207, 200530707, 205029206, 209530705, 214035205, 218542707,
            223053212, 227566721, 232083235, 236602755, 241125282, 245650817, 250179361, 254710915,
            259245480, 263783057, 268323647, 272867251, 277413870, 281963505, 286516157, 291071827,
            295630516, 300192225, 304756955, 309324707, 313895482, 318469281, 323046105, 327625955,
            332208832, 336794737, 341383671, 345975635, 350570630, 355168657, 359769717, 364373811,
            368980940, 373591105, 378204307, 382820547, 387439826, 392062145, 396687505, 401315907,
            405947352, 410581841, 415219375, 419859955, 424503582, 429150257, 433799981, 438452755,
            443108580, 447767457, 452429387, 457094371, 461762410, 466433505, 471107657, 475784867,
            480465136, 485148465, 489834855, 494524307, 499216822, 503912401, 508611045, 513312755,
            518017532, 522725377, 527436291, 532150275, 536867330, 541587457, 546310657, 551036931,
            555766280, 560498705, 565234207, 569972787, 574714446, 579459185, 584207005, 588957907,
            593711892, 598468961, 603229115, 607992355, 612758682, 617528097, 622300601, 627076195,
            631854880, 636636657, 641421527, 646209491, 651000550, 655794705, 660591957, 665392307,
            670195756, 675002305, 679811955, 684624707, 689440562, 694259521, 699081585, 703906755,
            708735032, 713566417, 718400911, 723238515, 728079230, 732923057, 737769997, 742620051,
            747473220, 752329505, 757188907, 762051427, 766917066, 771785825, 776657705, 781532707,
            786410832, 791292081, 796176455, 801063955, 805954582, 810848337, 815745221, 820645235,
            825548380, 830454657, 835364067, 840276611, 845192290, 850111105, 855033057, 859958147,
            864886376, 869817745, 874752255, 879689907, 884630702, 889574641, 894521725, 899471955,
            904425332, 909381857, 914341531, 919304355, 924270330, 929239457, 934211737, 939187171,
            944165760, 949147505, 954132407, 959120467, 964111686, 969106065, 974103605, 979104307,
            984108172, 989115201, 994125395, 999138755, 1004155282, 1009174977, 1014197841,
            1019223875, 1024253080, 1029285457, 1034321007, 1039359731, 1044401630, 1049446705,
            1054494957, 1059546387, 1064600996, 1069658785, 1074719755, 1079783907, 1084851242,
            1089921761, 1094995465, 1100072355, 1105152432, 1110235697, 1115322151, 1120411795,
            1125504630, 1130600657, 1135699877, 1140802291, 1145907900, 1151016705, 1156128707,
            1161243907, 1166362306, 1171483905, 1176608705, 1181736707, 1186867912, 1192002321,
            1197139935, 1202280755, 1207424782, 1212572017, 1217722461, 1222876115, 1228032980,
            1233193057, 1238356347, 1243522851, 1248692570, 1253865505, 1259041657, 1264221027,
            1269403616, 1274589425, 1279778455, 1284970707, 1290166182, 1295364881, 1300566805,
            1305771955, 1310980332, 1316191937, 1321406771, 1326624835, 1331846130, 1337070657,
            1342298417, 1347529411, 1352763640, 1358001105, 1363241807, 1368485747, 1373732926,
            1378983345, 1384237005, 1389493907, 1394754052, 1400017441, 1405284075, 1410553955,
            1415827082, 1421103457, 1426383081, 1431665955, 1436952080, 1442241457, 1447534087,
            1452829971, 1458129110, 1463431505, 1468737157, 1474046067, 1479358236, 1484673665,
            1489992355, 1495314307, 1500639522, 1505968001, 1511299745, 1516634755, 1521973032,
            1527314577, 1532659391, 1538007475, 1543358830, 1548713457, 1554071357, 1559432531,
            1564796980, 1570164705, 1575535707, 1580909987, 1586287546, 1591668385, 1597052505,
            1602439907, 1607830592, 1613224561, 1618621815, 1624022355, 1629426182, 1634833297,
            1640243701, 1645657395, 1651074380, 1656494657, 1661918227, 1667345091, 1672775250,
            1678208705, 1683645457, 1689085507, 1694528856, 1699975505, 1705425455, 1710878707,
            1716335262, 1721795121, 1727258285, 1732724755, 1738194532, 1743667617, 1749144011,
            1754623715, 1760106730, 1765593057, 1771082697, 1776575651, 1782071920, 1787571505,
            1793074407, 1798580627, 1804090166, 1809603025, 1815119205, 1820638707, 1826161532,
            1831687681, 1837217155, 1842749955, 1848286082, 1853825537, 1859368321, 1864914435,
            1870463880, 1876016657, 1881572767, 1887132211, 1892694990, 1898261105, 1903830557,
            1909403347, 1914979476, 1920558945, 1926141755, 1931727907, 1937317402, 1942910241,
            1948506425, 1954105955, 1959708832, 1965315057, 1970924631, 1976537555, 1982153830,
            1987773457, 1993396437, 1999022771, 2004652460, 2010285505, 2015921907, 2021561667,
            2027204786, 2032851265, 2038501105, 2044154307, 2049810872, 2055470801, 2061134095,
            2066800755, 2072470782, 2078144177, 2083820941, 2089501075, 2095184580, 2100871457,
            2106561707, 2112255331, 2117952330, 2123652705, 2129356457, 2135063587, 2140774096,
            2146487985, 2152205255, 2157925907, 2163649942, 2169377361, 2175108165, 2180842355,
            2186579932, 2192320897, 2198065251, 2203812995, 2209564130, 2215318657, 2221076577,
            2226837891, 2232602600, 2238370705, 2244142207, 2249917107, 2255695406, 2261477105,
            2267262205, 2273050707, 2278842612, 2284637921, 2290436635, 2296238755, 2302044282,
            2307853217, 2313665561, 2319481315, 2325300480, 2331123057, 2336949047, 2342778451,
            2348611270, 2354447505, 2360287157, 2366130227, 2371976716, 2377826625, 2383679955,
            2389536707, 2395396882, 2401260481, 2407127505, 2412997955, 2418871832, 2424749137,
            2430629871, 2436514035, 2442401630, 2448292657, 2454187117, 2460085011, 2465986340,
            2471891105, 2477799307, 2483710947, 2489626026, 2495544545, 2501466505, 2507391907,
            2513320752, 2519253041, 2525188775, 2531127955, 2537070582, 2543016657, 2548966181,
            2554919155, 2560875580, 2566835457, 2572798787, 2578765571, 2584735810, 2590709505,
            2596686657, 2602667267, 2608651336, 2614638865, 2620629855, 2626624307, 2632622222,
            2638623601, 2644628445, 2650636755, 2656648532, 2662663777, 2668682491, 2674704675,
            2680730330, 2686759457, 2692792057, 2698828131, 2704867680, 2710910705, 2716957207,
            2723007187, 2729060646, 2735117585, 2741178005, 2747241907, 2753309292, 2759380161,
            2765454515, 2771532355, 2777613682, 2783698497, 2789786801, 2795878595, 2801973880,
            2808072657, 2814174927, 2820280691, 2826389950, 2832502705, 2838618957, 2844738707,
            2850861956, 2856988705, 2863118955, 2869252707, 2875389962, 2881530721, 2887674985,
            2893822755, 2899974032, 2906128817, 2912287111, 2918448915, 2924614230, 2930783057,
            2936955397, 2943131251, 2949310620, 2955493505, 2961679907, 2967869827, 2974063266,
            2980260225, 2986460705, 2992664707, 2998872232, 3005083281, 3011297855, 3017515955,
            3023737582, 3029962737, 3036191421, 3042423635, 3048659380, 3054898657, 3061141467,
            3067387811, 3073637690, 3079891105, 3086148057, 3092408547, 3098672576, 3104940145,
            3111211255, 3117485907, 3123764102, 3130045841, 3136331125, 3142619955, 3148912332,
            3155208257, 3161507731, 3167810755, 3174117330, 3180427457, 3186741137, 3193058371,
            3199379160, 3205703505, 3212031407, 3218362867, 3224697886, 3231036465, 3237378605,
            3243724307, 3250073572, 3256426401, 3262782795, 3269142755, 3275506282, 3281873377,
            3288244041, 3294618275, 3300996080, 3307377457, 3313762407, 3320150931, 3326543030,
            3332938705, 3339337957, 3345740787, 3352147196, 3358557185, 3364970755, 3371387907,
            3377808642, 3384232961, 3390660865, 3397092355, 3403527432, 3409966097, 3416408351,
            3422854195, 3429303630, 3435756657, 3442213277, 3448673491, 3455137300, 3461604705,
            3468075707, 3474550307, 3481028506, 3487510305, 3493995705, 3500484707, 3506977312,
            3513473521, 3519973335, 3526476755, 3532983782, 3539494417, 3546008661, 3552526515,
            3559047980, 3565573057, 3572101747, 3578634051, 3585169970, 3591709505, 3598252657,
            3604799427, 3611349816, 3617903825, 3624461455, 3631022707, 3637587582, 3644156081,
            3650728205, 3657303955, 3663883332, 3670466337, 3677052971, 3683643235, 3690237130,
            3696834657, 3703435817, 3710040611, 3716649040, 3723261105, 3729876807, 3736496147,
            3743119126, 3749745745, 3756376005, 3763009907, 3769647452, 3776288641, 3782933475,
            3789581955, 3796234082, 3802889857, 3809549281, 3816212355, 3822879080, 3829549457,
            3836223487, 3842901171, 3849582510, 3856267505, 3862956157, 3869648467, 3876344436,
            3883044065, 3889747355, 3896454307, 3903164922, 3909879201, 3916597145, 3923318755,
            3930044032, 3936772977, 3943505591, 3950241875, 3956981830, 3963725457, 3970472757,
            3977223731, 3983978380, 3990736705, 3997498707, 4004264387, 4011033746, 4017806785,
            4024583505, 4031363907, 4038147992, 4044935761, 4051727215, 4058522355, 4065321182,
            4072123697, 4078929901, 4085739795, 4092553380, 4099370657, 4106191627, 4113016291,
            4119844650, 4126676705, 4133512457, 4140351907, 4147195056, 4154041905, 4160892455,
            4167746707, 4174604662, 4181466321, 4188331685, 4195200755, 4202073532, 4208950017,
            4215830211, 4222714115, 4229601730, 4236493057, 4243388097, 4250286851, 4257189320,
            4264095505, 4271005407, 4277919027, 4284836366, 4291757425, 3714909, 10643412,
            17575637, 24511586, 31451260, 38394660, 45341787, 52292642, 59247226, 66205540,
            73167585, 80133362, 87102872, 94076116, 101053095, 108033810, 115018262, 122006452,
            128998381, 135994050, 142993460, 149996612, 157003507, 164014146, 171028530, 178046660,
            185068537, 192094162, 199123536, 206156660, 213193535, 220234162, 227278542, 234326676,
            241378565, 248434210, 255493612, 262556772, 269623691, 276694370, 283768810, 290847012,
            297928977, 305014706, 312104200, 319197460, 326294487, 333395282, 340499846, 347608180,
            354720285, 361836162, 368955812, 376079236, 383206435, 390337410, 397472162, 404610692,
            411753001, 418899090, 426048960, 433202612, 440360047, 447521266, 454686270, 461855060,
            469027637, 476204002, 483384156, 490568100, 497755835, 504947362, 512142682, 519341796,
            526544705, 533751410, 540961912, 548176212, 555394311, 562616210, 569841910, 577071412,
            584304717, 591541826, 598782740, 606027460, 613275987, 620528322, 627784466, 635044420,
            642308185, 649575762, 656847152, 664122356, 671401375, 678684210, 685970862, 693261332,
            700555621, 707853730, 715155660, 722461412, 729770987, 737084386, 744401610, 751722660,
            759047537, 766376242, 773708776, 781045140, 788385335, 795729362, 803077222, 810428916,
            817784445, 825143810, 832507012, 839874052, 847244931, 854619650, 861998210, 869380612,
            876766857, 884156946, 891550880, 898948660, 906350287, 913755762, 921165086, 928578260,
            935995285, 943416162, 950840892, 958269476, 965701915, 973138210, 980578362, 988022372,
            995470241, 1002921970, 1010377560, 1017837012, 1025300327, 1032767506, 1040238550,
            1047713460, 1055192237, 1062674882, 1070161396, 1077651780, 1085146035, 1092644162,
            1100146162, 1107652036, 1115161785, 1122675410, 1130192912, 1137714292, 1145239551,
            1152768690, 1160301710, 1167838612, 1175379397, 1182924066, 1190472620, 1198025060,
            1205581387, 1213141602, 1220705706, 1228273700, 1235845585, 1243421362, 1251001032,
            1258584596, 1266172055, 1273763410, 1281358662, 1288957812, 1296560861, 1304167810,
            1311778660, 1319393412, 1327012067, 1334634626, 1342261090, 1349891460, 1357525737,
            1365163922, 1372806016, 1380452020, 1388101935, 1395755762, 1403413502, 1411075156,
            1418740725, 1426410210, 1434083612, 1441760932, 1449442171, 1457127330, 1464816410,
            1472509412, 1480206337, 1487907186, 1495611960, 1503320660, 1511033287, 1518749842,
            1526470326, 1534194740, 1541923085, 1549655362, 1557391572, 1565131716, 1572875795,
            1580623810, 1588375762, 1596131652, 1603891481, 1611655250, 1619422960, 1627194612,
            1634970207, 1642749746, 1650533230, 1658320660, 1666112037, 1673907362, 1681706636,
            1689509860, 1697317035, 1705128162, 1712943242, 1720762276, 1728585265, 1736412210,
            1744243112, 1752077972, 1759916791, 1767759570, 1775606310, 1783457012, 1791311677,
            1799170306, 1807032900, 1814899460, 1822769987, 1830644482, 1838522946, 1846405380,
            1854291785, 1862182162, 1870076512, 1877974836, 1885877135, 1893783410, 1901693662,
            1909607892, 1917526101, 1925448290, 1933374460, 1941304612, 1949238747, 1957176866,
            1965118970, 1973065060, 1981015137, 1988969202, 1996927256, 2004889300, 2012855335,
            2020825362, 2028799382, 2036777396, 2044759405, 2052745410, 2060735412, 2068729412,
            2076727411, 2084729410, 2092735410, 2100745412, 2108759417, 2116777426, 2124799440,
            2132825460, 2140855487, 2148889522, 2156927566, 2164969620, 2173015685, 2181065762,
            2189119852, 2197177956, 2205240075, 2213306210, 2221376362, 2229450532, 2237528721,
            2245610930, 2253697160, 2261787412, 2269881687, 2277979986, 2286082310, 2294188660,
            2302299037, 2310413442, 2318531876, 2326654340, 2334780835, 2342911362, 2351045922,
            2359184516, 2367327145, 2375473810, 2383624512, 2391779252, 2399938031, 2408100850,
            2416267710, 2424438612, 2432613557, 2440792546, 2448975580, 2457162660, 2465353787,
            2473548962, 2481748186, 2489951460, 2498158785, 2506370162, 2514585592, 2522805076,
            2531028615, 2539256210, 2547487862, 2555723572, 2563963341, 2572207170, 2580455060,
            2588707012, 2596963027, 2605223106, 2613487250, 2621755460, 2630027737, 2638304082,
            2646584496, 2654868980, 2663157535, 2671450162, 2679746862, 2688047636, 2696352485,
            2704661410, 2712974412, 2721291492, 2729612651, 2737937890, 2746267210, 2754600612,
            2762938097, 2771279666, 2779625320, 2787975060, 2796328887, 2804686802, 2813048806,
            2821414900, 2829785085, 2838159362, 2846537732, 2854920196, 2863306755, 2871697410,
            2880092162, 2888491012, 2896893961, 2905301010, 2913712160, 2922127412, 2930546767,
            2938970226, 2947397790, 2955829460, 2964265237, 2972705122, 2981149116, 2989597220,
            2998049435, 3006505762, 3014966202, 3023430756, 3031899425, 3040372210, 3048849112,
            3057330132, 3065815271, 3074304530, 3082797910, 3091295412, 3099797037, 3108302786,
            3116812660, 3125326660, 3133844787, 3142367042, 3150893426, 3159423940, 3167958585,
            3176497362, 3185040272, 3193587316, 3202138495, 3210693810, 3219253262, 3227816852,
            3236384581, 3244956450, 3253532460, 3262112612, 3270696907, 3279285346, 3287877930,
            3296474660, 3305075537, 3313680562, 3322289736, 3330903060, 3339520535, 3348142162,
            3356767942, 3365397876, 3374031965, 3382670210, 3391312612, 3399959172, 3408609891,
            3417264770, 3425923810, 3434587012, 3443254377, 3451925906, 3460601600, 3469281460,
            3477965487, 3486653682, 3495346046, 3504042580, 3512743285, 3521448162, 3530157212,
            3538870436, 3547587835, 3556309410, 3565035162, 3573765092, 3582499201, 3591237490,
            3599979960, 3608726612, 3617477447, 3626232466, 3634991670, 3643755060, 3652522637,
            3661294402, 3670070356, 3678850500, 3687634835, 3696423362, 3705216082, 3714012996,
            3722814105, 3731619410, 3740428912, 3749242612, 3758060511, 3766882610, 3775708910,
            3784539412, 3793374117, 3802213026, 3811056140, 3819903460, 3828754987, 3837610722,
            3846470666, 3855334820, 3864203185, 3873075762, 3881952552, 3890833556, 3899718775,
            3908608210, 3917501862, 3926399732, 3935301821, 3944208130, 3953118660, 3962033412,
            3970952387, 3979875586, 3988803010, 3997734660, 4006670537, 4015610642, 4024554976,
            4033503540, 4042456335, 4051413362, 4060374622, 4069340116, 4078309845, 4087283810,
            4096262012, 4105244452, 4114231131, 4123222050, 4132217210, 4141216612, 4150220257,
            4159228146, 4168240280, 4177256660, 4186277287, 4195302162, 4204331286, 4213364660,
            4222402285, 4231444162, 4240490292, 4249540676, 4258595315, 4267654210, 4276717362,
            4285784772, 4294856441, 8965074, 18045265, 27129717, 36218432, 45311411, 54408655,
            63510165, 72615942, 81725987, 90840301, 99958885, 109081740, 118208867, 127340267,
            136475941, 145615890, 154760115, 163908617, 173061397, 182218456, 191379795, 200545415,
            209715317, 218889502, 228067971, 237250725, 246437765, 255629092, 264824707, 274024611,
            283228805, 292437290, 301650067, 310867137, 320088501, 329314160, 338544115, 347778367,
            357016917, 366259766, 375506915, 384758365, 394014117, 403274172, 412538531, 421807195,
            431080165, 440357442, 449639027, 458924921, 468215125, 477509640, 486808467, 496111607,
            505419061, 514730830, 524046915, 533367317, 542692037, 552021076, 561354435, 570692115,
            580034117, 589380442, 598731091, 608086065, 617445365, 626808992, 636176947, 645549231,
            654925845, 664306790, 673692067, 683081677, 692475621, 701873900, 711276515, 720683467,
            730094757, 739510386, 748930355, 758354665, 767783317, 777216312, 786653651, 796095335,
            805541365, 814991742, 824446467, 833905541, 843368965, 852836740, 862308867, 871785347,
            881266181, 890751370, 900240915, 909734817, 919233077, 928735696, 938242675, 947754015,
            957269717, 966789782, 976314211, 985843005, 995376165, 1004913692, 1014455587,
            1024001851, 1033552485, 1043107490, 1052666867, 1062230617, 1071798741, 1081371240,
            1090948115, 1100529367, 1110114997, 1119705006, 1129299395, 1138898165, 1148501317,
            1158108852, 1167720771, 1177337075, 1186957765, 1196582842, 1206212307, 1215846161,
            1225484405, 1235127040, 1244774067, 1254425487, 1264081301, 1273741510, 1283406115,
            1293075117, 1302748517, 1312426316, 1322108515, 1331795115, 1341486117, 1351181522,
            1360881331, 1370585545, 1380294165, 1390007192, 1399724627, 1409446471, 1419172725,
            1428903390, 1438638467, 1448377957, 1458121861, 1467870180, 1477622915, 1487380067,
            1497141637, 1506907626, 1516678035, 1526452865, 1536232117, 1546015792, 1555803891,
            1565596415, 1575393365, 1585194742, 1595000547, 1604810781, 1614625445, 1624444540,
            1634268067, 1644096027, 1653928421, 1663765250, 1673606515, 1683452217, 1693302357,
            1703156936, 1713015955, 1722879415, 1732747317, 1742619662, 1752496451, 1762377685,
            1772263365, 1782153492, 1792048067, 1801947091, 1811850565, 1821758490, 1831670867,
            1841587697, 1851508981, 1861434720, 1871364915, 1881299567, 1891238677, 1901182246,
            1911130275, 1921082765, 1931039717, 1941001132, 1950967011, 1960937355, 1970912165,
            1980891442, 1990875187, 2000863401, 2010856085, 2020853240, 2030854867, 2040860967,
            2050871541, 2060886590, 2070906115, 2080930117, 2090958597, 2100991556, 2111028995,
            2121070915, 2131117317, 2141168202, 2151223571, 2161283425, 2171347765, 2181416592,
            2191489907, 2201567711, 2211650005, 2221736790, 2231828067, 2241923837, 2252024101,
            2262128860, 2272238115, 2282351867, 2292470117, 2302592866, 2312720115, 2322851865,
            2332988117, 2343128872, 2353274131, 2363423895, 2373578165, 2383736942, 2393900227,
            2404068021, 2414240325, 2424417140, 2434598467, 2444784307, 2454974661, 2465169530,
            2475368915, 2485572817, 2495781237, 2505994176, 2516211635, 2526433615, 2536660117,
            2546891142, 2557126691, 2567366765, 2577611365, 2587860492, 2598114147, 2608372331,
            2618635045, 2628902290, 2639174067, 2649450377, 2659731221, 2670016600, 2680306515,
            2690600967, 2700899957, 2711203486, 2721511555, 2731824165, 2742141317, 2752463012,
            2762789251, 2773120035, 2783455365, 2793795242, 2804139667, 2814488641, 2824842165,
            2835200240, 2845562867, 2855930047, 2866301781, 2876678070, 2887058915, 2897444317,
            2907834277, 2918228796, 2928627875, 2939031515, 2949439717, 2959852482, 2970269811,
            2980691705, 2991118165, 3001549192, 3011984787, 3022424951, 3032869685, 3043318990,
            3053772867, 3064231317, 3074694341, 3085161940, 3095634115, 3106110867, 3116592197,
            3127078106, 3137568595, 3148063665, 3158563317, 3169067552, 3179576371, 3190089775,
            3200607765, 3211130342, 3221657507, 3232189261, 3242725605, 3253266540, 3263812067,
            3274362187, 3284916901, 3295476210, 3306040115, 3316608617, 3327181717, 3337759416,
            3348341715, 3358928615, 3369520117, 3380116222, 3390716931, 3401322245, 3411932165,
            3422546692, 3433165827, 3443789571, 3454417925, 3465050890, 3475688467, 3486330657,
            3496977461, 3507628880, 3518284915, 3528945567, 3539610837, 3550280726, 3560955235,
            3571634365, 3582318117, 3593006492, 3603699491, 3614397115, 3625099365, 3635806242,
            3646517747, 3657233881, 3667954645, 3678680040, 3689410067, 3700144727, 3710884021,
            3721627950, 3732376515, 3743129717, 3753887557, 3764650036, 3775417155, 3786188915,
            3796965317, 3807746362, 3818532051, 3829322385, 3840117365, 3850916992, 3861721267,
            3872530191, 3883343765, 3894161990, 3904984867, 3915812397, 3926644581, 3937481420,
            3948322915, 3959169067, 3970019877, 3980875346, 3991735475, 4002600265, 4013469717,
            4024343832, 4035222611, 4046106055, 4056994165, 4067886942, 4078784387, 4089686501,
            4100593285, 4111504740, 4122420867, 4133341667, 4144267141, 4155197290, 4166132115,
            4177071617, 4188015797, 4198964656, 4209918195, 4220876415, 4231839317, 4242806902,
            4253779171, 4264756125, 4275737765, 4286724092, 2747811, 13743516, 24743910, 35748995,
            46758772, 57773242, 68792406, 79816265, 90844820, 101878072, 112916022, 123958671,
            135006020, 146058070, 157114822, 168176277, 179242436, 190313300, 201388870, 212469147,
            223554132, 234643826, 245738230, 256837345, 267941172, 279049712, 290162966, 301280935,
            312403620, 323531022, 334663142, 345799981, 356941540, 368087820, 379238822, 390394547,
            401554996, 412720170, 423890070, 435064697, 446244052, 457428136, 468616950, 479810495,
            491008772, 502211782, 513419526, 524632005, 535849220, 547071172, 558297862, 569529291,
            580765460, 592006370, 603252022, 614502417, 625757556, 637017440, 648282070, 659551447,
            670825572, 682104446, 693388070, 704676445, 715969572, 727267452, 738570086, 749877475,
            761189620, 772506522, 783828182, 795154601, 806485780, 817821720, 829162422, 840507887,
            851858116, 863213110, 874572870, 885937397, 897306692, 908680756, 920059590, 931443195,
            942831572, 954224722, 965622646, 977025345, 988432820, 999845072, 1011262102,
            1022683911, 1034110500, 1045541870, 1056978022, 1068418957, 1079864676, 1091315180,
            1102770470, 1114230547, 1125695412, 1137165066, 1148639510, 1160118745, 1171602772,
            1183091592, 1194585206, 1206083615, 1217586820, 1229094822, 1240607622, 1252125221,
            1263647620, 1275174820, 1286706822, 1298243627, 1309785236, 1321331650, 1332882870,
            1344438897, 1355999732, 1367565376, 1379135830, 1390711095, 1402291172, 1413876062,
            1425465766, 1437060285, 1448659620, 1460263772, 1471872742, 1483486531, 1495105140,
            1506728570, 1518356822, 1529989897, 1541627796, 1553270520, 1564918070, 1576570447,
            1588227652, 1599889686, 1611556550, 1623228245, 1634904772, 1646586132, 1658272326,
            1669963355, 1681659220, 1693359922, 1705065462, 1716775841, 1728491060, 1740211120,
            1751936022, 1763665767, 1775400356, 1787139790, 1798884070, 1810633197, 1822387172,
            1834145996, 1845909670, 1857678195, 1869451572, 1881229802, 1893012886, 1904800825,
            1916593620, 1928391272, 1940193782, 1952001151, 1963813380, 1975630470, 1987452422,
            1999279237, 2011110916, 2022947460, 2034788870, 2046635147, 2058486292, 2070342306,
            2082203190, 2094068945, 2105939572, 2117815072, 2129695446, 2141580695, 2153470820,
            2165365822, 2177265702, 2189170461, 2201080100, 2212994620, 2224914022, 2236838307,
            2248767476, 2260701530, 2272640470, 2284584297, 2296533012, 2308486616, 2320445110,
            2332408495, 2344376772, 2356349942, 2368328006, 2380310965, 2392298820, 2404291572,
            2416289222, 2428291771, 2440299220, 2452311570, 2464328822, 2476350977, 2488378036,
            2500410000, 2512446870, 2524488647, 2536535332, 2548586926, 2560643430, 2572704845,
            2584771172, 2596842412, 2608918566, 2620999635, 2633085620, 2645176522, 2657272342,
            2669373081, 2681478740, 2693589320, 2705704822, 2717825247, 2729950596, 2742080870,
            2754216070, 2766356197, 2778501252, 2790651236, 2802806150, 2814965995, 2827130772,
            2839300482, 2851475126, 2863654705, 2875839220, 2888028672, 2900223062, 2912422391,
            2924626660, 2936835870, 2949050022, 2961269117, 2973493156, 2985722140, 2997956070,
            3010194947, 3022438772, 3034687546, 3046941270, 3059199945, 3071463572, 3083732152,
            3096005686, 3108284175, 3120567620, 3132856022, 3145149382, 3157447701, 3169750980,
            3182059220, 3194372422, 3206690587, 3219013716, 3231341810, 3243674870, 3256012897,
            3268355892, 3280703856, 3293056790, 3305414695, 3317777572, 3330145422, 3342518246,
            3354896045, 3367278820, 3379666572, 3392059302, 3404457011, 3416859700, 3429267370,
            3441680022, 3454097657, 3466520276, 3478947880, 3491380470, 3503818047, 3516260612,
            3528708166, 3541160710, 3553618245, 3566080772, 3578548292, 3591020806, 3603498315,
            3615980820, 3628468322, 3640960822, 3653458321, 3665960820, 3678468320, 3690980822,
            3703498327, 3716020836, 3728548350, 3741080870, 3753618397, 3766160932, 3778708476,
            3791261030, 3803818595, 3816381172, 3828948762, 3841521366, 3854098985, 3866681620,
            3879269272, 3891861942, 3904459631, 3917062340, 3929670070, 3942282822, 3954900597,
            3967523396, 3980151220, 3992784070, 4005421947, 4018064852, 4030712786, 4043365750,
            4056023745, 4068686772, 4081354832, 4094027926, 4106706055, 4119389220, 4132077422,
            4144770662, 4157468941, 4170172260, 4182880620, 4195594022, 4208312467, 4221035956,
            4233764490, 4246498070, 4259236697, 4271980372, 4284729096, 2515574, 15274400,
            28038277, 40807207, 53581191, 66360230, 79144325, 91933477, 104727687, 117526956,
            130331285, 143140675, 155955127, 168774642, 181599221, 194428865, 207263575, 220103352,
            232948197, 245798111, 258653095, 271513150, 284378277, 297248477, 310123751, 323004100,
            335889525, 348780027, 361675607, 374576266, 387482005, 400392825, 413308727, 426229712,
            439155781, 452086935, 465023175, 477964502, 490910917, 503862421, 516819015, 529780700,
            542747477, 555719347, 568696311, 581678370, 594665525, 607657777, 620655127, 633657576,
            646665125, 659677775, 672695527, 685718382, 698746341, 711779405, 724817575, 737860852,
            750909237, 763962731, 777021335, 790085050, 803153877, 816227817, 829306871, 842391040,
            855480325, 868574727, 881674247, 894778886, 907888645, 921003525, 934123527, 947248652,
            960378901, 973514275, 986654775, 999800402, 1012951157, 1026107041, 1039268055,
            1052434200, 1065605477, 1078781887, 1091963431, 1105150110, 1118341925, 1131538877,
            1144740967, 1157948196, 1171160565, 1184378075, 1197600727, 1210828522, 1224061461,
            1237299545, 1250542775, 1263791152, 1277044677, 1290303351, 1303567175, 1316836150,
            1330110277, 1343389557, 1356673991, 1369963580, 1383258325, 1396558227, 1409863287,
            1423173506, 1436488885, 1449809425, 1463135127, 1476465992, 1489802021, 1503143215,
            1516489575, 1529841102, 1543197797, 1556559661, 1569926695, 1583298900, 1596676277,
            1610058827, 1623446551, 1636839450, 1650237525, 1663640777, 1677049207, 1690462816,
            1703881605, 1717305575, 1730734727, 1744169062, 1757608581, 1771053285, 1784503175,
            1797958252, 1811418517, 1824883971, 1838354615, 1851830450, 1865311477, 1878797697,
            1892289111, 1905785720, 1919287525, 1932794527, 1946306727, 1959824126, 1973346725,
            1986874525, 2000407527, 2013945732, 2027489141, 2041037755, 2054591575, 2068150602,
            2081714837, 2095284281, 2108858935, 2122438800, 2136023877, 2149614167, 2163209671,
            2176810390, 2190416325, 2204027477, 2217643847, 2231265436, 2244892245, 2258524275,
            2272161527, 2285804002, 2299451701, 2313104625, 2326762775, 2340426152, 2354094757,
            2367768591, 2381447655, 2395131950, 2408821477, 2422516237, 2436216231, 2449921460,
            2463631925, 2477347627, 2491068567, 2504794746, 2518526165, 2532262825, 2546004727,
            2559751872, 2573504261, 2587261895, 2601024775, 2614792902, 2628566277, 2642344901,
            2656128775, 2669917900, 2683712277, 2697511907, 2711316791, 2725126930, 2738942325,
            2752762977, 2766588887, 2780420056, 2794256485, 2808098175, 2821945127, 2835797342,
            2849654821, 2863517565, 2877385575, 2891258852, 2905137397, 2919021211, 2932910295,
            2946804650, 2960704277, 2974609177, 2988519351, 3002434800, 3016355525, 3030281527,
            3044212807, 3058149366, 3072091205, 3086038325, 3099990727, 3113948412, 3127911381,
            3141879635, 3155853175, 3169832002, 3183816117, 3197805521, 3211800215, 3225800200,
            3239805477, 3253816047, 3267831911, 3281853070, 3295879525, 3309911277, 3323948327,
            3337990676, 3352038325, 3366091275, 3380149527, 3394213082, 3408281941, 3422356105,
            3436435575, 3450520352, 3464610437, 3478705831, 3492806535, 3506912550, 3521023877,
            3535140517, 3549262471, 3563389740, 3577522325, 3591660227, 3605803447, 3619951986,
            3634105845, 3648265025, 3662429527, 3676599352, 3690774501, 3704954975, 3719140775,
            3733331902, 3747528357, 3761730141, 3775937255, 3790149700, 3804367477, 3818590587,
            3832819031, 3847052810, 3861291925, 3875536377, 3889786167, 3904041296, 3918301765,
            3932567575, 3946838727, 3961115222, 3975397061, 3989684245, 4003976775, 4018274652,
            4032577877, 4046886451, 4061200375, 4075519650, 4089844277, 4104174257, 4118509591,
            4132850280, 4147196325, 4161547727, 4175904487, 4190266606, 4204634085, 4219006925,
            4233385127, 4247768692, 4262157621, 4276551915, 4290951575, 10389306, 24799702,
            39215466, 53636600, 68063105, 82494982, 96932232, 111374856, 125822855, 140276230,
            154734982, 169199112, 183668621, 198143510, 212623780, 227109432, 241600467, 256096886,
            270598690, 285105880, 299618457, 314136422, 328659776, 343188520, 357722655, 372262182,
            386807102, 401357416, 415913125, 430474230, 445040732, 459612632, 474189931, 488772630,
            503360730, 517954232, 532553137, 547157446, 561767160, 576382280, 591002807, 605628742,
            620260086, 634896840, 649539005, 664186582, 678839572, 693497976, 708161795, 722831030,
            737505682, 752185752, 766871241, 781562150, 796258480, 810960232, 825667407, 840380006,
            855098030, 869821480, 884550357, 899284662, 914024396, 928769560, 943520155, 958276182,
            973037642, 987804536, 1002576865, 1017354630, 1032137832, 1046926472, 1061720551,
            1076520070, 1091325030, 1106135432, 1120951277, 1135772566, 1150599300, 1165431480,
            1180269107, 1195112182, 1209960706, 1224814680, 1239674105, 1254538982, 1269409312,
            1284285096, 1299166335, 1314053030, 1328945182, 1343842792, 1358745861, 1373654390,
            1388568380, 1403487832, 1418412747, 1433343126, 1448278970, 1463220280, 1478167057,
            1493119302, 1508077016, 1523040200, 1538008855, 1552982982, 1567962582, 1582947656,
            1597938205, 1612934230, 1627935732, 1642942712, 1657955171, 1672973110, 1687996530,
            1703025432, 1718059817, 1733099686, 1748145040, 1763195880, 1778252207, 1793314022,
            1808381326, 1823454120, 1838532405, 1853616182, 1868705452, 1883800216, 1898900475,
            1914006230, 1929117482, 1944234232, 1959356481, 1974484230, 1989617480, 2004756232,
            2019900487, 2035050246, 2050205510, 2065366280, 2080532557, 2095704342, 2110881636,
            2126064440, 2141252755, 2156446582, 2171645922, 2186850776, 2202061145, 2217277030,
            2232498432, 2247725352, 2262957791, 2278195750, 2293439230, 2308688232, 2323942757,
            2339202806, 2354468380, 2369739480, 2385016107, 2400298262, 2415585946, 2430879160,
            2446177905, 2461482182, 2476791992, 2492107336, 2507428215, 2522754630, 2538086582,
            2553424072, 2568767101, 2584115670, 2599469780, 2614829432, 2630194627, 2645565366,
            2660941650, 2676323480, 2691710857, 2707103782, 2722502256, 2737906280, 2753315855,
            2768730982, 2784151662, 2799577896, 2815009685, 2830447030, 2845889932, 2861338392,
            2876792411, 2892251990, 2907717130, 2923187832, 2938664097, 2954145926, 2969633320,
            2985126280, 3000624807, 3016128902, 3031638566, 3047153800, 3062674605, 3078200982,
            3093732932, 3109270456, 3124813555, 3140362230, 3155916482, 3171476312, 3187041721,
            3202612710, 3218189280, 3233771432, 3249359167, 3264952486, 3280551390, 3296155880,
            3311765957, 3327381622, 3343002876, 3358629720, 3374262155, 3389900182, 3405543802,
            3421193016, 3436847825, 3452508230, 3468174232, 3483845832, 3499523031, 3515205830,
            3530894230, 3546588232, 3562287837, 3577993046, 3593703860, 3609420280, 3625142307,
            3640869942, 3656603186, 3672342040, 3688086505, 3703836582, 3719592272, 3735353576,
            3751120495, 3766893030, 3782671182, 3798454952, 3814244341, 3830039350, 3845839980,
            3861646232, 3877458107, 3893275606, 3909098730, 3924927480, 3940761857, 3956601862,
            3972447496, 3988298760, 4004155655, 4020018182, 4035886342, 4051760136, 4067639565,
            4083524630, 4099415332, 4115311672, 4131213651, 4147121270, 4163034530, 4178953432,
            4194877977, 4210808166, 4226744000, 4242685480, 4258632607, 4274585382, 4290543806,
            11540584, 27510310, 43485687, 59466717, 75453401, 91445740, 107443735, 123447387,
            139456697, 155471666, 171492295, 187518585, 203550537, 219588152, 235631431, 251680375,
            267734985, 283795262, 299861207, 315932821, 332010105, 348093060, 364181687, 380275987,
            396375961, 412481610, 428592935, 444709937, 460832617, 476960976, 493095015, 509234735,
            525380137, 541531222, 557687991, 573850445, 590018585, 606192412, 622371927, 638557131,
            654748025, 670944610, 687146887, 703354857, 719568521, 735787880, 752012935, 768243687,
            784480137, 800722286, 816970135, 833223685, 849482937, 865747892, 882018551, 898294915,
            914576985, 930864762, 947158247, 963457441, 979762345, 996072960, 1012389287,
            1028711327, 1045039081, 1061372550, 1077711735, 1094056637, 1110407257, 1126763596,
            1143125655, 1159493435, 1175866937, 1192246162, 1208631111, 1225021785, 1241418185,
            1257820312, 1274228167, 1290641751, 1307061065, 1323486110, 1339916887, 1356353397,
            1372795641, 1389243620, 1405697335, 1422156787, 1438621977, 1455092906, 1471569575,
            1488051985, 1504540137, 1521034032, 1537533671, 1554039055, 1570550185, 1587067062,
            1603589687, 1620118061, 1636652185, 1653192060, 1669737687, 1686289067, 1702846201,
            1719409090, 1735977735, 1752552137, 1769132297, 1785718216, 1802309895, 1818907335,
            1835510537, 1852119502, 1868734231, 1885354725, 1901980985, 1918613012, 1935250807,
            1951894371, 1968543705, 1985198810, 2001859687, 2018526337, 2035198761, 2051876960,
            2068560935, 2085250687, 2101946217, 2118647526, 2135354615, 2152067485, 2168786137,
            2185510572, 2202240791, 2218976795, 2235718585, 2252466162, 2269219527, 2285978681,
            2302743625, 2319514360, 2336290887, 2353073207, 2369861321, 2386655230, 2403454935,
            2420260437, 2437071737, 2453888836, 2470711735, 2487540435, 2504374937, 2521215242,
            2538061351, 2554913265, 2571770985, 2588634512, 2605503847, 2622378991, 2639259945,
            2656146710, 2673039287, 2689937677, 2706841881, 2723751900, 2740667735, 2757589387,
            2774516857, 2791450146, 2808389255, 2825334185, 2842284937, 2859241512, 2876203911,
            2893172135, 2910146185, 2927126062, 2944111767, 2961103301, 2978100665, 2995103860,
            3012112887, 3029127747, 3046148441, 3063174970, 3080207335, 3097245537, 3114289577,
            3131339456, 3148395175, 3165456735, 3182524137, 3199597382, 3216676471, 3233761405,
            3250852185, 3267948812, 3285051287, 3302159611, 3319273785, 3336393810, 3353519687,
            3370651417, 3387789001, 3404932440, 3422081735, 3439236887, 3456397897, 3473564766,
            3490737495, 3507916085, 3525100537, 3542290852, 3559487031, 3576689075, 3593896985,
            3611110762, 3628330407, 3645555921, 3662787305, 3680024560, 3697267687, 3714516687,
            3731771561, 3749032310, 3766298935, 3783571437, 3800849817, 3818134076, 3835424215,
            3852720235, 3870022137, 3887329922, 3904643591, 3921963145, 3939288585, 3956619912,
            3973957127, 3991300231, 4008649225, 4026004110, 4043364887, 4060731557, 4078104121,
            4095482580, 4112866935, 4130257187, 4147653337, 4165055386, 4182463335, 4199877185,
            4217296937, 4234722592, 4252154151, 4269591615, 4287034985, 9516966, 26972152,
            44433246, 61900250, 79373165, 96851992, 114336732, 131827386, 149323955, 166826440,
            184334842, 201849162, 219369401, 236895560, 254427640, 271965642, 289509567, 307059416,
            324615190, 342176890, 359744517, 377318072, 394897556, 412482970, 430074315, 447671592,
            465274802, 482883946, 500499025, 518120040, 535746992, 553379882, 571018711, 588663480,
            606314190, 623970842, 641633437, 659301976, 676976460, 694656890, 712343267, 730035592,
            747733866, 765438090, 783148265, 800864392, 818586472, 836314506, 854048495, 871788440,
            889534342, 907286202, 925044021, 942807800, 960577540, 978353242, 996134907,
            1013922536, 1031716130, 1049515690, 1067321217, 1085132712, 1102950176, 1120773610,
            1138603015, 1156438392, 1174279742, 1192127066, 1209980365, 1227839640, 1245704892,
            1263576122, 1281453331, 1299336520, 1317225690, 1335120842, 1353021977, 1370929096,
            1388842200, 1406761290, 1424686367, 1442617432, 1460554486, 1478497530, 1496446565,
            1514401592, 1532362612, 1550329626, 1568302635, 1586281640, 1604266642, 1622257642,
            1640254641, 1658257640, 1676266640, 1694281642, 1712302647, 1730329656, 1748362670,
            1766401690, 1784446717, 1802497752, 1820554796, 1838617850, 1856686915, 1874761992,
            1892843082, 1910930186, 1929023305, 1947122440, 1965227592, 1983338762, 2001455951,
            2019579160, 2037708390, 2055843642, 2073984917, 2092132216, 2110285540, 2128444890,
            2146610267, 2164781672, 2182959106, 2201142570, 2219332065, 2237527592, 2255729152,
            2273936746, 2292150375, 2310370040, 2328595742, 2346827482, 2365065261, 2383309080,
            2401558940, 2419814842, 2438076787, 2456344776, 2474618810, 2492898890, 2511185017,
            2529477192, 2547775416, 2566079690, 2584390015, 2602706392, 2621028822, 2639357306,
            2657691845, 2676032440, 2694379092, 2712731802, 2731090571, 2749455400, 2767826290,
            2786203242, 2804586257, 2822975336, 2841370480, 2859771690, 2878178967, 2896592312,
            2915011726, 2933437210, 2951868765, 2970306392, 2988750092, 3007199866, 3025655715,
            3044117640, 3062585642, 3081059722, 3099539881, 3118026120, 3136518440, 3155016842,
            3173521327, 3192031896, 3210548550, 3229071290, 3247600117, 3266135032, 3284676036,
            3303223130, 3321776315, 3340335592, 3358900962, 3377472426, 3396049985, 3414633640,
            3433223392, 3451819242, 3470421191, 3489029240, 3507643390, 3526263642, 3544889997,
            3563522456, 3582161020, 3600805690, 3619456467, 3638113352, 3656776346, 3675445450,
            3694120665, 3712801992, 3731489432, 3750182986, 3768882655, 3787588440, 3806300342,
            3825018362, 3843742501, 3862472760, 3881209140, 3899951642, 3918700267, 3937455016,
            3956215890, 3974982890, 3993756017, 4012535272, 4031320656, 4050112170, 4068909815,
            4087713592, 4106523502, 4125339546, 4144161725, 4162990040, 4181824492, 4200665082,
            4219511811, 4238364680, 4257223690, 4276088842, 4294960137, 18870280, 37753865,
            56643595, 75539472, 94441497, 113349671, 132263995, 151184470, 170111097, 189043877,
            207982811, 226927900, 245879145, 264836547, 283800107, 302769826, 321745705, 340727745,
            359715947, 378710312, 397710841, 416717535, 435730395, 454749422, 473774617, 492805981,
            511843515, 530887220, 549937097, 568993147, 588055371, 607123770, 626198345, 645279097,
            664366027, 683459136, 702558425, 721663895, 740775547, 759893382, 779017401, 798147605,
            817283995, 836426572, 855575337, 874730291, 893891435, 913058770, 932232297, 951412017,
            970597931, 989790040, 1008988345, 1028192847, 1047403547, 1066620446, 1085843545,
            1105072845, 1124308347, 1143550052, 1162797961, 1182052075, 1201312395, 1220578922,
            1239851657, 1259130601, 1278415755, 1297707120, 1317004697, 1336308487, 1355618491,
            1374934710, 1394257145, 1413585797, 1432920667, 1452261756, 1471609065, 1490962595,
            1510322347, 1529688322, 1549060521, 1568438945, 1587823595, 1607214472, 1626611577,
            1646014911, 1665424475, 1684840270, 1704262297, 1723690557, 1743125051, 1762565780,
            1782012745, 1801465947, 1820925387, 1840391066, 1859862985, 1879341145, 1898825547,
            1918316192, 1937813081, 1957316215, 1976825595, 1996341222, 2015863097, 2035391221,
            2054925595, 2074466220, 2094013097, 2113566227, 2133125611, 2152691250, 2172263145,
            2191841297, 2211425707, 2231016376, 2250613305, 2270216495, 2289825947, 2309441662,
            2329063641, 2348691885, 2368326395, 2387967172, 2407614217, 2427267531, 2446927115,
            2466592970, 2486265097, 2505943497, 2525628171, 2545319120, 2565016345, 2584719847,
            2604429627, 2624145686, 2643868025, 2663596645, 2683331547, 2703072732, 2722820201,
            2742573955, 2762333995, 2782100322, 2801872937, 2821651841, 2841437035, 2861228520,
            2881026297, 2900830367, 2920640731, 2940457390, 2960280345, 2980109597, 2999945147,
            3019786996, 3039635145, 3059489595, 3079350347, 3099217402, 3119090761, 3138970425,
            3158856395, 3178748672, 3198647257, 3218552151, 3238463355, 3258380870, 3278304697,
            3298234837, 3318171291, 3338114060, 3358063145, 3378018547, 3397980267, 3417948306,
            3437922665, 3457903345, 3477890347, 3497883672, 3517883321, 3537889295, 3557901595,
            3577920222, 3597945177, 3617976461, 3638014075, 3658058020, 3678108297, 3698164907,
            3718227851, 3738297130, 3758372745, 3778454697, 3798542987, 3818637616, 3838738585,
            3858845895, 3878959547, 3899079542, 3919205881, 3939338565, 3959477595, 3979622972,
            3999774697, 4019932771, 4040097195, 4060267970, 4080445097, 4100628577, 4120818411,
            4141014600, 4161217145, 4181426047, 4201641307, 4221862926, 4242090905, 4262325245,
            4282565947, 7845716, 28099146, 48358940, 68625100, 88897627, 109176522, 129461786,
            149753420, 170051425, 190355802, 210666552, 230983676, 251307175, 271637050, 291973302,
            312315932, 332664941, 353020330, 373382100, 393750252, 414124787, 434505706, 454893010,
            475286700, 495686777, 516093242, 536506096, 556925340, 577350975, 597783002, 618221422,
            638666236, 659117445, 679575050, 700039052, 720509452, 740986251, 761469450, 781959050,
            802455052, 822957457, 843466266, 863981480, 884503100, 905031127, 925565562, 946106406,
            966653660, 987207325, 1007767402, 1028333892, 1048906796, 1069486115, 1090071850,
            1110664002, 1131262572, 1151867561, 1172478970, 1193096800, 1213721052, 1234351727,
            1254988826, 1275632350, 1296282300, 1316938677, 1337601482, 1358270716, 1378946380,
            1399628475, 1420317002, 1441011962, 1461713356, 1482421185, 1503135450, 1523856152,
            1544583292, 1565316871, 1586056890, 1606803350, 1627556252, 1648315597, 1669081386,
            1689853620, 1710632300, 1731417427, 1752209002, 1773007026, 1793811500, 1814622425,
            1835439802, 1856263632, 1877093916, 1897930655, 1918773850, 1939623502, 1960479612,
            1981342181, 2002211210, 2023086700, 2043968652, 2064857067, 2085751946, 2106653290,
            2127561100, 2148475377, 2169396122, 2190323336, 2211257020, 2232197175, 2253143802,
            2274096902, 2295056476, 2316022525, 2336995050, 2357974052, 2378959532, 2399951491,
            2420949930, 2441954850, 2462966252, 2483984137, 2505008506, 2526039360, 2547076700,
            2568120527, 2589170842, 2610227646, 2631290940, 2652360725, 2673437002, 2694519772,
            2715609036, 2736704795, 2757807050, 2778915802, 2800031052, 2821152801, 2842281050,
            2863415800, 2884557052, 2905704807, 2926859066, 2948019830, 2969187100, 2990360877,
            3011541162, 3032727956, 3053921260, 3075121075, 3096327402, 3117540242, 3138759596,
            3159985465, 3181217850, 3202456752, 3223702172, 3244954111, 3266212570, 3287477550,
            3308749052, 3330027077, 3351311626, 3372602700, 3393900300, 3415204427, 3436515082,
            3457832266, 3479155980, 3500486225, 3521823002, 3543166312, 3564516156, 3585872535,
            3607235450, 3628604902, 3649980892, 3671363421, 3692752490, 3714148100, 3735550252,
            3756958947, 3778374186, 3799795970, 3821224300, 3842659177, 3864100602, 3885548576,
            3907003100, 3928464175, 3949931802, 3971405982, 3992886716, 4014374005, 4035867850,
            4057368252, 4078875212, 4100388731, 4121908810, 4143435450, 4164968652, 4186508417,
            4208054746, 4229607640, 4251167100, 4272733127, 4294305722, 20917590, 42503325,
            64095630, 85694507, 107299957, 128911981, 150530580, 172155755, 193787507, 215425837,
            237070746, 258722235, 280380305, 302044957, 323716192, 345394011, 367078415, 388769405,
            410466982, 432171147, 453881901, 475599245, 497323180, 519053707, 540790827, 562534541,
            584284850, 606041755, 627805257, 649575357, 671352056, 693135355, 714925255, 736721757,
            758524862, 780334571, 802150885, 823973805, 845803332, 867639467, 889482211, 911331565,
            933187530, 955050107, 976919297, 998795101, 1020677520, 1042566555, 1064462207,
            1086364477, 1108273366, 1130188875, 1152111005, 1174039757, 1195975132, 1217917131,
            1239865755, 1261821005, 1283782882, 1305751387, 1327726521, 1349708285, 1371696680,
            1393691707, 1415693367, 1437701661, 1459716590, 1481738155, 1503766357, 1525801197,
            1547842676, 1569890795, 1591945555, 1614006957, 1636075002, 1658149691, 1680231025,
            1702319005, 1724413632, 1746514907, 1768622831, 1790737405, 1812858630, 1834986507,
            1857121037, 1879262221, 1901410060, 1923564555, 1945725707, 1967893517, 1990067986,
            2012249115, 2034436905, 2056631357, 2078832472, 2101040251, 2123254695, 2145475805,
            2167703582, 2189938027, 2212179141, 2234426925, 2256681380, 2278942507, 2301210307,
            2323484781, 2345765930, 2368053755, 2390348257, 2412649437, 2434957296, 2457271835,
            2479593055, 2501920957, 2524255542, 2546596811, 2568944765, 2591299405, 2613660732,
            2636028747, 2658403451, 2680784845, 2703172930, 2725567707, 2747969177, 2770377341,
            2792792200, 2815213755, 2837642007, 2860076957, 2882518606, 2904966955, 2927422005,
            2949883757, 2972352212, 2994827371, 3017309235, 3039797805, 3062293082, 3084795067,
            3107303761, 3129819165, 3152341280, 3174870107, 3197405647, 3219947901, 3242496870,
            3265052555, 3287614957, 3310184077, 3332759916, 3355342475, 3377931755, 3400527757,
            3423130482, 3445739931, 3468356105, 3490979005, 3513608632, 3536244987, 3558888071,
            3581537885, 3604194430, 3626857707, 3649527717, 3672204461, 3694887940, 3717578155,
            3740275107, 3762978797, 3785689226, 3808406395, 3831130305, 3853860957, 3876598352,
            3899342491, 3922093375, 3944851005, 3967615382, 3990386507, 4013164381, 4035949005,
            4058740380, 4081538507, 4104343387, 4127155021, 4149973410, 4172798555, 4195630457,
            4218469117, 4241314536, 4264166715, 4287025655, 14924061, 37796527, 60675756, 83561750,
            106454510, 129354037, 152260332, 175173396, 198093230, 221019835, 243953212, 266893362,
            289840286, 312793985, 335754460, 358721712, 381695742, 404676551, 427664140, 450658510,
            473659662, 496667597, 519682316, 542703820, 565732110, 588767187, 611809052, 634857706,
            657913150, 680975385, 704044412, 727120232, 750202846, 773292255, 796388460, 819491462,
            842601262, 865717861, 888841260, 911971460, 935108462, 958252267, 981402876,
            1004560290, 1027724510, 1050895537, 1074073372, 1097258016, 1120449470, 1143647735,
            1166852812, 1190064702, 1213283406, 1236508925, 1259741260, 1282980412, 1306226382,
            1329479171, 1352738780, 1376005210, 1399278462, 1422558537, 1445845436, 1469139160,
            1492439710, 1515747087, 1539061292, 1562382326, 1585710190, 1609044885, 1632386412,
            1655734772, 1679089966, 1702451995, 1725820860, 1749196562, 1772579102, 1795968481,
            1819364700, 1842767760, 1866177662, 1889594407, 1913017996, 1936448430, 1959885710,
            1983329837, 2006780812, 2030238636, 2053703310, 2077174835, 2100653212, 2124138442,
            2147630526, 2171129465, 2194635260, 2218147912, 2241667422, 2265193791, 2288727020,
            2312267110, 2335814062, 2359367877, 2382928556, 2406496100, 2430070510, 2453651787,
            2477239932, 2500834946, 2524436830, 2548045585, 2571661212, 2595283712, 2618913086,
            2642549335, 2666192460, 2689842462, 2713499342, 2737163101, 2760833740, 2784511260,
            2808195662, 2831886947, 2855585116, 2879290170, 2903002110, 2926720937, 2950446652,
            2974179256, 2997918750, 3021665135, 3045418412, 3069178582, 3092945646, 3116719605,
            3140500460, 3164288212, 3188082862, 3211884411, 3235692860, 3259508210, 3283330462,
            3307159617, 3330995676, 3354838640, 3378688510, 3402545287, 3426408972, 3450279566,
            3474157070, 3498041485, 3521932812, 3545831052, 3569736206, 3593648275, 3617567260,
            3641493162, 3665425982, 3689365721, 3713312380, 3737265960, 3761226462, 3785193887,
            3809168236, 3833149510, 3857137710, 3881132837, 3905134892, 3929143876, 3953159790,
            3977182635, 4001212412, 4025249122, 4049292766, 4073343345, 4097400860, 4121465312,
            4145536702, 4169615031, 4193700300, 4217792510, 4241891662, 4265997757, 4290110796,
            19263484, 43390415, 67524292, 91665117, 115812891, 139967615, 164129290, 188297917,
            212473497, 236656031, 260845520, 285041965, 309245367, 333455727, 357673046, 381897325,
            406128565, 430366767, 454611932, 478864061, 503123155, 527389215, 551662242, 575942237,
            600229201, 624523135, 648824040, 673131917, 697446767, 721768591, 746097390, 770433165,
            794775917, 819125647, 843482356, 867846045, 892216715, 916594367, 940979002, 965370621,
            989769225, 1014174815, 1038587392, 1063006957, 1087433511, 1111867055, 1136307590,
            1160755117, 1185209637, 1209671151, 1234139660, 1258615165, 1283097667, 1307587167,
            1332083666, 1356587165, 1381097665, 1405615167, 1430139672, 1454671181, 1479209695,
            1503755215, 1528307742, 1552867277, 1577433821, 1602007375, 1626587940, 1651175517,
            1675770107, 1700371711, 1724980330, 1749595965, 1774218617, 1798848287, 1823484976,
            1848128685, 1872779415, 1897437167, 1922101942, 1946773741, 1971452565, 1996138415,
            2020831292, 2045531197, 2070238131, 2094952095, 2119673090, 2144401117, 2169136177,
            2193878271, 2218627400, 2243383565, 2268146767, 2292917007, 2317694286, 2342478605,
            2367269965, 2392068367, 2416873812, 2441686301, 2466505835, 2491332415, 2516166042,
            2541006717, 2565854441, 2590709215, 2615571040, 2640439917, 2665315847, 2690198831,
            2715088870, 2739985965, 2764890117, 2789801327, 2814719596, 2839644925, 2864577315,
            2889516767, 2914463282, 2939416861, 2964377505, 2989345215, 3014319992, 3039301837,
            3064290751, 3089286735, 3114289790, 3139299917, 3164317117, 3189341391, 3214372740,
            3239411165, 3264456667, 3289509247, 3314568906, 3339635645, 3364709465, 3389790367,
            3414878352, 3439973421, 3465075575, 3490184815, 3515301142, 3540424557, 3565555061,
            3590692655, 3615837340, 3640989117, 3666147987, 3691313951, 3716487010, 3741667165,
            3766854417, 3792048767, 3817250216, 3842458765, 3867674415, 3892897167, 3918127022,
            3943363981, 3968608045, 3993859215, 4019117492, 4044382877, 4069655371, 4094934975,
            4120221690, 4145515517, 4170816457, 4196124511, 4221439680, 4246761965, 4272091367,
            2460591, 27804231, 53154990, 78512870, 103877872, 129249997, 154629246, 180015620,
            205409120, 230809747, 256217502, 281632386, 307054400, 332483545, 357919822, 383363232,
            408813776, 434271455, 459736270, 485208222, 510687312, 536173541, 561666910, 587167420,
            612675072, 638189867, 663711806, 689240890, 714777120, 740320497, 765871022, 791428696,
            816993520, 842565495, 868144622, 893730902, 919324336, 944924925, 970532670, 996147572,
            1021769632, 1047398851, 1073035230, 1098678770, 1124329472, 1149987337, 1175652366,
            1201324560, 1227003920, 1252690447, 1278384142, 1304085006, 1329793040, 1355508245,
            1381230622, 1406960172, 1432696896, 1458440795, 1484191870, 1509950122, 1535715552,
            1561488161, 1587267950, 1613054920, 1638849072, 1664650407, 1690458926, 1716274630,
            1742097520, 1767927597, 1793764862, 1819609316, 1845460960, 1871319795, 1897185822,
            1923059042, 1948939456, 1974827065, 2000721870, 2026623872, 2052533072, 2078449471,
            2104373070, 2130303870, 2156241872, 2182187077, 2208139486, 2234099100, 2260065920,
            2286039947, 2312021182, 2338009626, 2364005280, 2390008145, 2416018222, 2442035512,
            2468060016, 2494091735, 2520130670, 2546176822, 2572230192, 2598290781, 2624358590,
            2650433620, 2676515872, 2702605347, 2728702046, 2754805970, 2780917120, 2807035497,
            2833161102, 2859293936, 2885434000, 2911581295, 2937735822, 2963897582, 2990066576,
            3016242805, 3042426270, 3068616972, 3094814912, 3121020091, 3147232510, 3173452170,
            3199679072, 3225913217, 3252154606, 3278403240, 3304659120, 3330922247, 3357192622,
            3383470246, 3409755120, 3436047245, 3462346622, 3488653252, 3514967136, 3541288275,
            3567616670, 3593952322, 3620295232, 3646645401, 3673002830, 3699367520, 3725739472,
            3752118687, 3778505166, 3804898910, 3831299920, 3857708197, 3884123742, 3910546556,
            3936976640, 3963413995, 3989858622, 4016310522, 4042769696, 4069236145, 4095709870,
            4122190872, 4148679152, 4175174711, 4201677550, 4228187670, 4254705072, 4281229757,
            12794430, 39333685, 65880225, 92434052, 118995167, 145563571, 172139265, 198722250,
            225312527, 251910097, 278514961, 305127120, 331746575, 358373327, 385007377, 411648726,
            438297375, 464953325, 491616577, 518287132, 544964991, 571650155, 598342625, 625042402,
            651749487, 678463881, 705185585, 731914600, 758650927, 785394567, 812145521, 838903790,
            865669375, 892442277, 919222497, 946010036, 972804895, 999607075, 1026416577,
            1053233402, 1080057551, 1106889025, 1133727825, 1160573952, 1187427407, 1214288191,
            1241156305, 1268031750, 1294914527, 1321804637, 1348702081, 1375606860, 1402518975,
            1429438427, 1456365217, 1483299346, 1510240815, 1537189625, 1564145777, 1591109272,
            1618080111, 1645058295, 1672043825, 1699036702, 1726036927, 1753044501, 1780059425,
            1807081700, 1834111327, 1861148307, 1888192641, 1915244330, 1942303375, 1969369777,
            1996443537, 2023524656, 2050613135, 2077708975, 2104812177, 2131922742, 2159040671,
            2186165965, 2213298625, 2240438652, 2267586047, 2294740811, 2321902945, 2349072450,
            2376249327, 2403433577, 2430625201, 2457824200, 2485030575, 2512244327, 2539465457,
            2566693966, 2593929855, 2621173125, 2648423777, 2675681812, 2702947231, 2730220035,
            2757500225, 2784787802, 2812082767, 2839385121, 2866694865, 2894012000, 2921336527,
            2948668447, 2976007761, 3003354470, 3030708575, 3058070077, 3085438977, 3112815276,
            3140198975, 3167590075, 3194988577, 3222394482, 3249807791, 3277228505, 3304656625,
            3332092152, 3359535087, 3386985431, 3414443185, 3441908350, 3469380927, 3496860917,
            3524348321, 3551843140, 3579345375, 3606855027, 3634372097, 3661896586, 3689428495,
            3716967825, 3744514577, 3772068752, 3799630351, 3827199375, 3854775825, 3882359702,
            3909951007, 3937549741, 3965155905, 3992769500, 4020390527, 4048018987, 4075654881,
            4103298210, 4130948975, 4158607177, 4186272817, 4213945896, 4241626415, 4269314375,
            2042481, 29745327, 57455616, 85173350, 112898530, 140631157, 168371232, 196118756,
            223873730, 251636155, 279406032, 307183362, 334968146, 362760385, 390560080, 418367232,
            446181842, 474003911, 501833440, 529670430, 557514882, 585366797, 613226176, 641093020,
            668967330, 696849107, 724738352, 752635066, 780539250, 808450905, 836370032, 864296632,
            892230706, 920172255, 948121280, 976077782, 1004041762, 1032013221, 1059992160,
            1087978580, 1115972482, 1143973867, 1171982736, 1199999090, 1228022930, 1256054257,
            1284093072, 1312139376, 1340193170, 1368254455, 1396323232, 1424399502, 1452483266,
            1480574525, 1508673280, 1536779532, 1564893282, 1593014531, 1621143280, 1649279530,
            1677423282, 1705574537, 1733733296, 1761899560, 1790073330, 1818254607, 1846443392,
            1874639686, 1902843490, 1931054805, 1959273632, 1987499972, 2015733826, 2043975195,
            2072224080, 2100480482, 2128744402, 2157015841, 2185294800, 2213581280, 2241875282,
            2270176807, 2298485856, 2326802430, 2355126530, 2383458157, 2411797312, 2440143996,
            2468498210, 2496859955, 2525229232, 2553606042, 2581990386, 2610382265, 2638781680,
            2667188632, 2695603122, 2724025151, 2752454720, 2780891830, 2809336482, 2837788677,
            2866248416, 2894715700, 2923190530, 2951672907, 2980162832, 3008660306, 3037165330,
            3065677905, 3094198032, 3122725712, 3151260946, 3179803735, 3208354080, 3236911982,
            3265477442, 3294050461, 3322631040, 3351219180, 3379814882, 3408418147, 3437028976,
            3465647370, 3494273330, 3522906857, 3551547952, 3580196616, 3608852850, 3637516655,
            3666188032, 3694866982, 3723553506, 3752247605, 3780949280, 3809658532, 3838375362,
            3867099771, 3895831760, 3924571330, 3953318482, 3982073217, 4010835536, 4039605440,
            4068382930, 4097168007, 4125960672, 4154760926, 4183568770, 4212384205, 4241207232,
            4270037852, 3908770, 32754580, 61607985, 90468987, 119337587, 148213786, 177097585,
            205988985, 234887987, 263794592, 292708801, 321630615, 350560035, 379497062, 408441697,
            437393941, 466353795, 495321260, 524296337, 553279027, 582269331, 611267250, 640272785,
            669285937, 698306707, 727335096, 756371105, 785414735, 814465987, 843524862, 872591361,
            901665485, 930747235, 959836612, 988933617, 1018038251, 1047150515, 1076270410,
            1105397937, 1134533097, 1163675891, 1192826320, 1221984385, 1251150087, 1280323427,
            1309504406, 1338693025, 1367889285, 1397093187, 1426304732, 1455523921, 1484750755,
            1513985235, 1543227362, 1572477137, 1601734561, 1630999635, 1660272360, 1689552737,
            1718840767, 1748136451, 1777439790, 1806750785, 1836069437, 1865395747, 1894729716,
            1924071345, 1953420635, 1982777587, 2012142202, 2041514481, 2070894425, 2100282035,
            2129677312, 2159080257, 2188490871, 2217909155, 2247335110, 2276768737, 2306210037,
            2335659011, 2365115660, 2394579985, 2424051987, 2453531667, 2483019026, 2512514065,
            2542016785, 2571527187, 2601045272, 2630571041, 2660104495, 2689645635, 2719194462,
            2748750977, 2778315181, 2807887075, 2837466660, 2867053937, 2896648907, 2926251571,
            2955861930, 2985479985, 3015105737, 3044739187, 3074380336, 3104029185, 3133685735,
            3163349987, 3193021942, 3222701601, 3252388965, 3282084035, 3311786812, 3341497297,
            3371215491, 3400941395, 3430675010, 3460416337, 3490165377, 3519922131, 3549686600,
            3579458785, 3609238687, 3639026307, 3668821646, 3698624705, 3728435485, 3758253987,
            3788080212, 3817914161, 3847755835, 3877605235, 3907462362, 3937327217, 3967199801,
            3997080115, 4026968160, 4056863937, 4086767447, 4116678691, 4146597670, 4176524385,
            4206458837, 4236401027, 4266350956, 1341329, 31306740, 61279892, 91260787, 121249426,
            151245810, 181249940, 211261817, 241281442, 271308816, 301343940, 331386815, 361437442,
            391495822, 421561956, 451635845, 481717490, 511806892, 541904052, 572008971, 602121650,
            632242090, 662370292, 692506257, 722649986, 752801480, 782960740, 813127767, 843302562,
            873485126, 903675460, 933873565, 964079442, 994293092, 1024514516, 1054743715,
            1084980690, 1115225442, 1145477972, 1175738281, 1206006370, 1236282240, 1266565892,
            1296857327, 1327156546, 1357463550, 1387778340, 1418100917, 1448431282, 1478769436,
            1509115380, 1539469115, 1569830642, 1600199962, 1630577076, 1660961985, 1691354690,
            1721755192, 1752163492, 1782579591, 1813003490, 1843435190, 1873874692, 1904321997,
            1934777106, 1965240020, 1995710740, 2026189267, 2056675602, 2087169746, 2117671700,
            2148181465, 2178699042, 2209224432, 2239757636, 2270298655, 2300847490, 2331404142,
            2361968612, 2392540901, 2423121010, 2453708940, 2484304692, 2514908267, 2545519666,
            2576138890, 2606765940, 2637400817, 2668043522, 2698694056, 2729352420, 2760018615,
            2790692642, 2821374502, 2852064196, 2882761725, 2913467090, 2944180292, 2974901332,
            3005630211, 3036366930, 3067111490, 3097863892, 3128624137, 3159392226, 3190168160,
            3220951940, 3251743567, 3282543042, 3313350366, 3344165540, 3374988565, 3405819442,
            3436658172, 3467504756, 3498359195, 3529221490, 3560091642, 3590969652, 3621855521,
            3652749250, 3683650840, 3714560292, 3745477607, 3776402786, 3807335830, 3838276740,
            3869225517, 3900182162, 3931146676, 3962119060, 3993099315, 4024087442, 4055083442,
            4086087316, 4117099065, 4148118690, 4179146192, 4210181572, 4241224831, 4272275970,
            8367694, 39434597, 70509382, 101592051, 132682605, 163781045, 194887372, 226001587,
            257123691, 288253685, 319391570, 350537347, 381691017, 412852581, 444022040, 475199395,
            506384647, 537577797, 568778846, 599987795, 631204645, 662429397, 693662052, 724902611,
            756151075, 787407445, 818671722, 849943907, 881224001, 912512005, 943807920, 975111747,
            1006423487, 1037743141, 1069070710, 1100406195, 1131749597, 1163100917, 1194460156,
            1225827315, 1257202395, 1288585397, 1319976322, 1351375171, 1382781945, 1414196645,
            1445619272, 1477049827, 1508488311, 1539934725, 1571389070, 1602851347, 1634321557,
            1665799701, 1697285780, 1728779795, 1760281747, 1791791637, 1823309466, 1854835235,
            1886368945, 1917910597, 1949460192, 1981017731, 2012583215, 2044156645, 2075738022,
            2107327347, 2138924621, 2170529845, 2202143020, 2233764147, 2265393227, 2297030261,
            2328675250, 2360328195, 2391989097, 2423657957, 2455334776, 2487019555, 2518712295,
            2550412997, 2582121662, 2613838291, 2645562885, 2677295445, 2709035972, 2740784467,
            2772540931, 2804305365, 2836077770, 2867858147, 2899646497, 2931442821, 2963247120,
            2995059395, 3026879647, 3058707877, 3090544086, 3122388275, 3154240445, 3186100597,
            3217968732, 3249844851, 3281728955, 3313621045, 3345521122, 3377429187, 3409345241,
            3441269285, 3473201320, 3505141347, 3537089367, 3569045381, 3601009390, 3632981395,
            3664961397, 3696949397, 3728945396, 3760949395, 3792961395, 3824981397, 3857009402,
            3889045411, 3921089425, 3953141445, 3985201472, 4017269507, 4049345551, 4081429605,
            4113521670, 4145621747, 4177729837, 4209845941, 4241970060, 4274102195, 11275051,
            43423222, 75579411, 107743620, 139915850, 172096102, 204284377, 236480676, 268685000,
            300897350, 333117727, 365346132, 397582566, 429827030, 462079525, 494340052, 526608612,
            558885206, 591169835, 623462500, 655763202, 688071942, 720388721, 752713540, 785046400,
            817387302, 849736247, 882093236, 914458270, 946831350, 979212477, 1011601652,
            1043998876, 1076404150, 1108817475, 1141238852, 1173668282, 1206105766, 1238551305,
            1271004900, 1303466552, 1335936262, 1368414031, 1400899860, 1433393750, 1465895702,
            1498405717, 1530923796, 1563449940, 1595984150, 1628526427, 1661076772, 1693635186,
            1726201670, 1758776225, 1791358852, 1823949552, 1856548326, 1889155175, 1921770100,
            1954393102, 1987024182, 2019663341, 2052310580, 2084965900, 2117629302, 2150300787,
            2182980356, 2215668010, 2248363750, 2281067577, 2313779492, 2346499496, 2379227590,
            2411963775, 2444708052, 2477460422, 2510220886, 2542989445, 2575766100, 2608550852,
            2641343702, 2674144651, 2706953700, 2739770850, 2772596102, 2805429457, 2838270916,
            2871120480, 2903978150, 2936843927, 2969717812, 3002599806, 3035489910, 3068388125,
            3101294452, 3134208892, 3167131446, 3200062115, 3233000900, 3265947802, 3298902822,
            3331865961, 3364837220, 3397816600, 3430804102, 3463799727, 3496803476, 3529815350,
            3562835350, 3595863477, 3628899732, 3661944116, 3694996630, 3728057275, 3761126052,
            3794202962, 3827288006, 3860381185, 3893482500, 3926591952, 3959709542, 3992835271,
            4025969140, 4059111150, 4092261302, 4125419597, 4158586036, 4191760620, 4224943350,
            4258134227, 4291333252, 29573130, 62788455, 96011930, 129243557, 162483337, 195731271,
            228987360, 262251605, 295524007, 328804567, 362093286, 395390165, 428695205, 462008407,
            495329772, 528659301, 561996995, 595342855, 628696882, 662059077, 695429441, 728807975,
            762194680, 795589557, 828992607, 862403831, 895823230, 929250805, 962686557, 996130487,
            1029582596, 1063042885, 1096511355, 1129988007, 1163472842, 1196965861, 1230467065,
            1263976455, 1297494032, 1331019797, 1364553751, 1398095895, 1431646230, 1465204757,
            1498771477, 1532346391, 1565929500, 1599520805, 1633120307, 1666728007, 1700343906,
            1733968005, 1767600305, 1801240807, 1834889512, 1868546421, 1902211535, 1935884855,
            1969566382, 2003256117, 2036954061, 2070660215, 2104374580, 2138097157, 2171827947,
            2205566951, 2239314170, 2273069605, 2306833257, 2340605127, 2374385216, 2408173525,
            2441970055, 2475774807, 2509587782, 2543408981, 2577238405, 2611076055, 2644921932,
            2678776037, 2712638371, 2746508935, 2780387730, 2814274757, 2848170017, 2882073511,
            2915985240, 2949905205, 2983833407, 3017769847, 3051714526, 3085667445, 3119628605,
            3153598007, 3187575652, 3221561541, 3255555675, 3289558055, 3323568682, 3357587557,
            3391614681, 3425650055, 3459693680, 3493745557, 3527805687, 3561874071, 3595950710,
            3630035605, 3664128757, 3698230167, 3732339836, 3766457765, 3800583955, 3834718407,
            3868861122, 3903012101, 3937171345, 3971338855, 4005514632, 4039698677, 4073890991,
            4108091575, 4142300430, 4176517557, 4210742957, 4244976631, 4279218580, 18501509,
            52760012, 87026792, 121301851, 155585190, 189876810, 224176712, 258484897, 292801366,
            327126120, 361459160, 395800487, 430150102, 464508006, 498874200, 533248685, 567631462,
            602022532, 636421896, 670829555, 705245510, 739669762, 774102312, 808543161, 842992310,
            877449760, 911915512, 946389567, 980871926, 1015362590, 1049861560, 1084368837,
            1118884422, 1153408316, 1187940520, 1222481035, 1257029862, 1291587002, 1326152456,
            1360726225, 1395308310, 1429898712, 1464497432, 1499104471, 1533719830, 1568343510,
            1602975512, 1637615837, 1672264486, 1706921460, 1741586760, 1776260387, 1810942342,
            1845632626, 1880331240, 1915038185, 1949753462, 1984477072, 2019209016, 2053949295,
            2088697910, 2123454862, 2158220152, 2192993781, 2227775750, 2262566060, 2297364712,
            2332171707, 2366987046, 2401810730, 2436642760, 2471483137, 2506331862, 2541188936,
            2576054360, 2610928135, 2645810262, 2680700742, 2715599576, 2750506765, 2785422310,
            2820346212, 2855278472, 2890219091, 2925168070, 2960125410, 2995091112, 3030065177,
            3065047606, 3100038400, 3135037560, 3170045087, 3205060982, 3240085246, 3275117880,
            3310158885, 3345208262, 3380266012, 3415332136, 3450406635, 3485489510, 3520580762,
            3555680392, 3590788401, 3625904790, 3661029560, 3696162712, 3731304247, 3766454166,
            3801612470, 3836779160, 3871954237, 3907137702, 3942329556, 3977529800, 4012738435,
            4047955462, 4083180882, 4118414696, 4153656905, 4188907510, 4224166512, 4259433912,
            4294709711, 35026614, 70319215, 105620217, 140929622, 176247431, 211573645, 246908265,
            282251292, 317602727, 352962571, 388330825, 423707490, 459092567, 494486057, 529887961,
            565298280, 600717015, 636144167, 671579737, 707023726, 742476135, 777936965, 813406217,
            848883892, 884369991, 919864515, 955367465, 990878842, 1026398647, 1061926881,
            1097463545, 1133008640, 1168562167, 1204124127, 1239694521, 1275273350, 1310860615,
            1346456317, 1382060457, 1417673036, 1453294055, 1488923515, 1524561417, 1560207762,
            1595862551, 1631525785, 1667197465, 1702877592, 1738566167, 1774263191, 1809968665,
            1845682590, 1881404967, 1917135797, 1952875081, 1988622820, 2024379015, 2060143667,
            2095916777, 2131698346, 2167488375, 2203286865, 2239093817, 2274909232, 2310733111,
            2346565455, 2382406265, 2418255542, 2454113287, 2489979501, 2525854185, 2561737340,
            2597628967, 2633529067, 2669437641, 2705354690, 2741280215, 2777214217, 2813156697,
            2849107656, 2885067095, 2921035015, 2957011417, 2992996302, 3028989671, 3064991525,
            3101001865, 3137020692, 3173048007, 3209083811, 3245128105, 3281180890, 3317242167,
            3353311937, 3389390201, 3425476960, 3461572215, 3497675967, 3533788217, 3569908966,
            3606038215, 3642175965, 3678322217, 3714476972, 3750640231, 3786811995, 3822992265,
            3859181042, 3895378327, 3931584121, 3967798425, 4004021240, 4040252567, 4076492407,
            4112740761, 4148997630, 4185263015, 4221536917, 4257819337, 4294110276, 35442439,
            71750420, 108066922, 144391947, 180725496, 217067570, 253418170, 289777297, 326144952,
            362521136, 398905850, 435299095, 471700872, 508111182, 544530026, 580957405, 617393320,
            653837772, 690290762, 726752291, 763222360, 799700970, 836188122, 872683817, 909188056,
            945700840, 982222170, 1018752047, 1055290472, 1091837446, 1128392970, 1164957045,
            1201529672, 1238110852, 1274700586, 1311298875, 1347905720, 1384521122, 1421145082,
            1457777601, 1494418680, 1531068320, 1567726522, 1604393287, 1641068616, 1677752510,
            1714444970, 1751145997, 1787855592, 1824573756, 1861300490, 1898035795, 1934779672,
            1971532122, 2008293146, 2045062745, 2081840920, 2118627672, 2155423002, 2192226911,
            2229039400, 2265860470, 2302690122, 2339528357, 2376375176, 2413230580, 2450094570,
            2486967147, 2523848312, 2560738066, 2597636410, 2634543345, 2671458872, 2708382992,
            2745315706, 2782257015, 2819206920, 2856165422, 2893132522, 2930108221, 2967092520,
            3004085420, 3041086922, 3078097027, 3115115736, 3152143050, 3189178970, 3226223497,
            3263276632, 3300338376, 3337408730, 3374487695, 3411575272, 3448671462, 3485776266,
            3522889685, 3560011720, 3597142372, 3634281642, 3671429531, 3708586040, 3745751170,
            3782924922, 3820107297, 3857298296, 3894497920, 3931706170, 3968923047, 4006148552,
            4043382686, 4080625450, 4117876845, 4155136872, 4192405532, 4229682826, 4266968755,
            9296024, 46599227, 83911067, 121231546, 158560665, 195898425, 233244827, 270599872,
            307963561, 345335895, 382716875, 420106502, 457504777, 494911701, 532327275, 569751500,
            607184377, 644625907, 682076091, 719534930, 757002425, 794478577, 831963387, 869456856,
            906958985, 944469775, 981989227, 1019517342, 1057054121, 1094599565, 1132153675,
            1169716452, 1207287897, 1244868011, 1282456795, 1320054250, 1357660377, 1395275177,
            1432898651, 1470530800, 1508171625, 1545821127, 1583479307, 1621146166, 1658821705,
            1696505925, 1734198827, 1771900412, 1809610681, 1847329635, 1885057275, 1922793602,
            1960538617, 1998292321, 2036054715, 2073825800, 2111605577, 2149394047, 2187191211,
            2224997070, 2262811625, 2300634877, 2338466827, 2376307476, 2414156825, 2452014875,
            2489881627, 2527757082, 2565641241, 2603534105, 2641435675, 2679345952, 2717264937,
            2755192631, 2793129035, 2831074150, 2869027977, 2906990517, 2944961771, 2982941740,
            3020930425, 3058927827, 3096933947, 3134948786, 3172972345, 3211004625, 3249045627,
            3287095352, 3325153801, 3363220975, 3401296875, 3439381502, 3477474857, 3515576941,
            3553687755, 3591807300, 3629935577, 3668072587, 3706218331, 3744372810, 3782536025,
            3820707977, 3858888667, 3897078096, 3935276265, 3973483175, 4011698827, 4049923222,
            4088156361, 4126398245, 4164648875, 4202908252, 4241176377, 4279453251, 22771579,
            61065955, 99369082, 137680962, 176001596, 214330985, 252669130, 291016032, 329371692,
            367736111, 406109290, 444491230, 482881932, 521281397, 559689626, 598106620, 636532380,
            674966907, 713410202, 751862266, 790323100, 828792705, 867271082, 905758232, 944254156,
            982758855, 1021272330, 1059794582, 1098325612, 1136865421, 1175414010, 1213971380,
            1252537532, 1291112467, 1329696186, 1368288690, 1406889980, 1445500057, 1484118922,
            1522746576, 1561383020, 1600028255, 1638682282, 1677345102, 1716016716, 1754697125,
            1793386330, 1832084332, 1870791132, 1909506731, 1948231130, 1986964330, 2025706332,
            2064457137, 2103216746, 2141985160, 2180762380, 2219548407, 2258343242, 2297146886,
            2335959340, 2374780605, 2413610682, 2452449572, 2491297276, 2530153795, 2569019130,
            2607893282, 2646776252, 2685668041, 2724568650, 2763478080, 2802396332, 2841323407,
            2880259306, 2919204030, 2958157580, 2997119957, 3036091162, 3075071196, 3114060060,
            3153057755, 3192064282, 3231079642, 3270103836, 3309136865, 3348178730, 3387229432,
            3426288972, 3465357351, 3504434570, 3543520630, 3582615532, 3621719277, 3660831866,
            3699953300, 3739083580, 3778222707, 3817370682, 3856527506, 3895693180, 3934867705,
            3974051082, 4013243312, 4052444396, 4091654335, 4130873130, 4170100782, 4209337292,
            4248582661, 4287836890, 32132684, 71404637, 110685452, 149975131, 189273675, 228581085,
            267897362, 307222507, 346556521, 385899405, 425251160, 464611787, 503981287, 543359661,
            582746910, 622143035, 661548037, 700961917, 740384676, 779816315, 819256835, 858706237,
            898164522, 937631691, 977107745, 1016592685, 1056086512, 1095589227, 1135100831,
            1174621325, 1214150710, 1253688987, 1293236157, 1332792221, 1372357180, 1411931035,
            1451513787, 1491105437, 1530705986, 1570315435, 1609933785, 1649561037, 1689197192,
            1728842251, 1768496215, 1808159085, 1847830862, 1887511547, 1927201141, 1966899645,
            2006607060, 2046323387, 2086048627, 2125782781, 2165525850, 2205277835, 2245038737,
            2284808557, 2324587296, 2364374955, 2404171535, 2443977037, 2483791462, 2523614811,
            2563447085, 2603288285, 2643138412, 2682997467, 2722865451, 2762742365, 2802628210,
            2842522987, 2882426697, 2922339341, 2962260920, 3002191435, 3042130887, 3082079277,
            3122036606, 3162002875, 3201978085, 3241962237, 3281955332, 3321957371, 3361968355,
            3401988285, 3442017162, 3482054987, 3522101761, 3562157485, 3602222160, 3642295787,
            3682378367, 3722469901, 3762570390, 3802679835, 3842798237, 3882925597, 3923061916,
            3963207195, 4003361435, 4043524637, 4083696802, 4123877931, 4164068025, 4204267085,
            4244475112, 4284692107, 29950775, 70185710, 110429615, 150682492, 190944342, 231215166,
            271494965, 311783740, 352081492, 392388222, 432703931, 473028620, 513362290, 553704942,
            594056577, 634417196, 674786800, 715165390, 755552967, 795949532, 836355086, 876769630,
            917193165, 957625692, 998067212, 1038517726, 1078977235, 1119445740, 1159923242,
            1200409742, 1240905241, 1281409740, 1321923240,
        ],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_low_same_length_fail_1() {
    let mut out = vec![10, 10, 10, 10, 10];
    limbs_mul_low_same_length(&mut out, &[6, 7], &[1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_low_same_length_fail_2() {
    let mut out = vec![10];
    limbs_mul_low_same_length(&mut out, &[6, 7], &[1, 2]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mul_low_same_length_fail_3() {
    let mut out = vec![10];
    limbs_mul_low_same_length(&mut out, &[], &[]);
}

#[test]
fn test_limbs_product() {
    fn test(xs: &[Limb], out: &[Limb]) {
        let xs_old = xs;
        let mut xs = xs_old.to_vec();
        let mut product = vec![0; xs.len()];
        let out_len = limbs_product(&mut product, &mut xs);
        product.truncate(out_len);
        assert_eq!(product, out);

        let xs = xs_old;
        let mut product_alt = vec![0; xs.len()];
        let out_len = limbs_product_naive(&mut product_alt, xs);
        product_alt.truncate(out_len);

        assert_eq!(
            Natural::from_owned_limbs_asc(product),
            Natural::from_owned_limbs_asc(product_alt)
        );
    }
    #[cfg(feature = "32_bit_limbs")]
    {
        test(&[0, 0], &[0]);
        test(&[0, 1], &[0]);
        test(&[1, 1], &[1]);
        test(&[1, 2, 3, 0], &[0]);
        test(&[1, 1, 1, 6], &[6]);
        // - factor_len < RECURSIVE_PROD_THRESHOLD
        // - factor_len < RECURSIVE_PROD_THRESHOLD && carry == 0
        test(&[2, 2], &[4]);
        // - 1 < factor_len < RECURSIVE_PROD_THRESHOLD
        test(&[2, 2, 2], &[8]);
        // - factor_len < RECURSIVE_PROD_THRESHOLD && carry != 0
        test(
            &[
                3364358997, 3754657515, 2983848742, 3936755874, 1784338974, 2546784265, 1325228501,
                2948540251,
            ],
            &[
                2931171496, 2374327460, 1603352486, 1643105815, 3729295616, 3598234472, 675706642,
                97731883,
            ],
        );
    }
    #[cfg(not(feature = "32_bit_limbs"))]
    {
        test(&[0, 0], &[0]);
        // - factor_len >= RECURSIVE_PROD_THRESHOLD
        // - factor_len >= RECURSIVE_PROD_THRESHOLD && carry != 0
        test(
            &[
                10008125746032292648,
                18321114734333840332,
                17920212203847604612,
                11217657766511696793,
                11281083380440747080,
                7892713234117558634,
                8902471323128328235,
                11892365124252837749,
                12611508814807140936,
                11815551321438701427,
                18110749687253320811,
                11715728409437156086,
                3339319126985623378,
                6325617411041006209,
                14096883611650388622,
                17687808067973538964,
                168019485471086672,
                15693590891786235960,
                2786725115760255375,
                9052512570921605652,
                14177547685319586247,
                15370386165587184937,
                5970865523433924174,
            ],
            &[
                11912117198576943104,
                13014631084302689641,
                13412132452197408442,
                2451810739915146054,
                11634661136866399370,
                4370855263334042992,
                15668998660816175865,
                12986191105652374961,
                13086723724779814479,
                11723085021234472016,
                11995103430870542887,
                9025449360203176092,
                2596804721077076476,
                17187504964195613394,
                3993660956642255072,
                14652834649000131068,
                115931129580828916,
                5908166419484799210,
                5266332857336541018,
                10618265378208305889,
                15818414605397825971,
                1340104142930176609,
                707009434669,
            ],
        );
        // - factor_len >= RECURSIVE_PROD_THRESHOLD && carry == 0
        test(
            &rle_decode(&[
                (0, 2),
                (18446743798831644672, 1),
                (u64::MAX, 8),
                (131071, 1),
                (0, 1),
                (18446181123756130304, 1),
                (u64::MAX, 1),
                (0, 3),
                (18446744073709289472, 1),
                (18446744069414584323, 1),
                (u64::MAX, 2),
                (1048575, 1),
            ]),
            &[0],
        );
        test(
            &[
                10985406156730349404,
                16998073653641921623,
                17457059674168741173,
                7286565767689019192,
                18125356898477139174,
                16492808515310020304,
                3477958611236241232,
                16292175871113024008,
                5585730561604558759,
                6236295300549613743,
                10319623255002503349,
                9463751064224151456,
                591089135516199459,
                2252218169318269834,
                862892606080353458,
                5048725452063866206,
                8818325570889918907,
                8659485206830156469,
                13423243087758132387,
                5931638651683836702,
                726189598260086702,
                17552568882310631283,
                16362554893644374308,
                2407211671412641092,
                1658933737262819201,
                8531670718492391711,
                7551068411167036177,
                1219676338169570619,
                3249808943561478386,
                8240095392791493806,
                18379760227315341655,
                15217742262236663404,
                17914601533857880122,
                6316977119306097487,
                15466746727219811764,
                3809403759956883034,
                7752635439402559334,
                18006879800705758675,
                10224737295140518487,
                8383030445894670697,
                7272423850473130597,
                9751703358656322718,
                5778325638584493526,
                8175215950976649646,
                8581067395248196883,
                4729909244293992358,
                7626677144783852491,
                15620921255634335582,
                17768397379068248272,
                7127187304413875110,
            ],
            &[
                3631027199567462400,
                1271721650030716672,
                2994855603235436973,
                17610775060915408482,
                5067312238234780182,
                17332419541318968246,
                16256455013715596255,
                8446753555477097340,
                6176580983302425584,
                4478791418516115226,
                4543632153902351949,
                8389257049215340154,
                7312665208898130485,
                7684632819757707617,
                3755281952906934158,
                13259632617670669038,
                8815229384280433597,
                18114656906332551194,
                8751551051855969605,
                3831140942508279269,
                3191248804852725269,
                2021644817234089610,
                17966341018397783935,
                17567823138927086650,
                7667876961419931279,
                14788070825593093177,
                18150799194320967018,
                10911257712565360499,
                17179583904099919127,
                13486844951716301158,
                10333473285718288690,
                11178175053564016056,
                7882508656916287058,
                10989425714502087490,
                6829028223894926894,
                810375652491815426,
                17216794810099974028,
                590650088582786587,
                4701269004129928787,
                3119542545671269984,
                14375368104481453295,
                124160740720125385,
                1360655591798967164,
                17850602821256829937,
                6139583463573727145,
                14578358562855390329,
                14560675482255305936,
                16662995492200800253,
                666535375735877288,
            ],
        );
    }
}

fn verify_limbs_mul_greater_to_out_fft(
    out_before: &[Limb],
    xs: &[Limb],
    ys: &[Limb],
    out_after: &[Limb],
) {
    let n = Natural::from_limbs_asc(xs) * Natural::from_limbs_asc(ys);
    let mut ns = n.into_limbs_asc();
    let len = xs.len() + ys.len();
    ns.resize(len, 0);
    assert_eq!(ns, &out_after[..len]);
    assert_eq!(&out_after[len..], &out_before[len..]);
}

#[test]
fn test_limbs_mul_greater_to_out_fft() {
    let test = |xs: &[Limb], ys: &[Limb], out_before: &[Limb], out_after: &[Limb]| {
        let mut out = out_before.to_vec();
        let mut scratch = vec![0; limbs_mul_greater_to_out_fft_scratch_len(xs.len(), ys.len())];
        limbs_mul_greater_to_out_fft(&mut out, xs, ys, &mut scratch);
        assert_eq!(out, out_after);
        let mut out = out_before.to_vec();
        let mut scratch =
            vec![0; limbs_mul_greater_to_out_fft_with_cutoff_scratch_len(xs.len(), ys.len(), 50)];
        limbs_mul_greater_to_out_fft_with_cutoff(&mut out, xs, ys, 50, &mut scratch);
        assert_eq!(out, out_after);

        verify_limbs_mul_greater_to_out_fft(out_before, xs, ys, out_after);
    };
    let test_big = |xs: &[Limb], ys: &[Limb]| {
        let mut out = vec![0; xs.len() + ys.len()];
        let out_before = out.clone();
        let mut scratch = vec![0; limbs_mul_greater_to_out_fft_scratch_len(xs.len(), ys.len())];
        limbs_mul_greater_to_out_fft(&mut out, xs, ys, &mut scratch);
        let out_after = out;
        let mut out = out_before.clone();
        let mut scratch =
            vec![0; limbs_mul_greater_to_out_fft_with_cutoff_scratch_len(xs.len(), ys.len(), 50)];
        limbs_mul_greater_to_out_fft_with_cutoff(&mut out, xs, ys, 50, &mut scratch);
        assert_eq!(out, out_after);

        verify_limbs_mul_greater_to_out_fft(&out_before, xs, ys, &out);
    };
    #[cfg(not(feature = "32_bit_limbs"))]
    {
        // - depth < 11
        // - depth < 6
        // - j1 + j2 - 1 <= 4 * n || w <= wadj
        // - j1 + j2 - 1 > 4 * n || w <= wadj
        // - trunc > n << 1 in limbs_mul_truncate_sqrt2
        // - top_bits != 0 in limbs_fft_split_bits
        // - i < length - 1 in limbs_split_bits_worker
        // - shift_bits == 0 in limbs_split_bits_worker
        // - shift_bits != 0 in limbs_split_bits_worker
        // - shift_bits >= Limb::WIDTH in limbs_split_bits_worker
        // - i >= length - 1 in limbs_split_bits_worker
        // - shift_bits != 0 in limbs_fft_split_bits
        // - w.even() in limbs_fft_truncate_sqrt2
        // - trunc == n << 1 in limbs_fft_truncate
        // - n != 1 in limbs_fft_radix2
        // - x == 0 in limbs_butterfly_lsh_b
        // - x == 0 && y == 0 in limbs_butterfly_lsh_b
        // - !xs.is_empty() in limbs_fft_sumdiff
        // - d == 0 in limbs_fft_mul_2expmod_2expp1_in_place
        // - x == 0 && y == 0 in limbs_butterfly_lsh_b
        // - !(sum ^ *x_lo).get_highest_bit() in limbs_fft_addmod_2expp1_1
        // - n == 1 in limbs_fft_radix2
        // - (sum ^ *x_lo).get_highest_bit() && c.get_highest_bit() in limbs_fft_addmod_2expp1_1
        // - h == 0 in limbs_fft_normmod_2expp1
        // - cy == 0 in limbs_fft_mulmod_2expp1_basecase_same
        // - cy == 0 && cz == 0 in limbs_fft_mulmod_2expp1_basecase_same
        // - h != 0 in limbs_fft_normmod_2expp1
        // - hi == 0 in limbs_fft_normmod_2expp1
        // - !(sum ^ *x_lo).get_highest_bit() && !c.get_highest_bit() in limbs_fft_addmod_2expp1_1
        // - w.even() in limbs_ifft_truncate_sqrt2
        // - trunc == n << 1 in limbs_ifft_truncate
        // - n != 1 in limbs_ifft_radix2
        // - n == 1 in limbs_ifft_radix2
        // - d == 0 in limbs_fft_div_2expmod_2expp1_in_place
        // - x == 0 in limbs_butterfly_rsh_b
        // - x == 0 && y == 0 in limbs_butterfly_rsh_b
        // - x == 0 && y != 0 in limbs_butterfly_rsh_b
        // - d != 0 in limbs_fft_div_2expmod_2expp1_in_place
        // - hi != 0 in limbs_fft_normmod_2expp1
        // - t[limbs] != Limb::MAX in limbs_fft_normmod_2expp1
        // - top_bits != 0 in limbs_fft_combine_bits
        // - shift_bits == 0 first time in limbs_fft_combine_bits
        // - shift_bits < Limb::WIDTH first time in limbs_fft_combine_bits
        // - shift_bits != 0 first time in limbs_fft_combine_bits
        // - shift_bits >= Limb::WIDTH first time in limbs_fft_combine_bits
        // - shift_bits != 0 second time in limbs_fft_combine_bits
        // - shift_bits >= Limb::WIDTH second time in limbs_fft_combine_bits
        test(
            &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            &[
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 1,
            ],
            &[10; 57],
            &[
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
                0,
            ],
        );
        test(
            &[10; 25],
            &[10; 32],
            &[100; 57],
            &[
                100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 1100, 1200, 1300, 1400, 1500,
                1600, 1700, 1800, 1900, 2000, 2100, 2200, 2300, 2400, 2500, 2500, 2500, 2500, 2500,
                2500, 2500, 2500, 2400, 2300, 2200, 2100, 2000, 1900, 1800, 1700, 1600, 1500, 1400,
                1300, 1200, 1100, 1000, 900, 800, 700, 600, 500, 400, 300, 200, 100, 0,
            ],
        );
        test(
            &[u64::MAX; 25],
            &[u64::MAX; 32],
            &[100; 57],
            &rle_decode(&[
                (1, 1),
                (0, 24),
                (u64::MAX, 7),
                (18446744073709551614, 1),
                (u64::MAX, 24),
            ]),
        );
        // - trunc != n << 1 && trunc > n in limbs_fft_truncate
        // - d != 0 in limbs_fft_mul_2expmod_2expp1_in_place
        // - x != 0 in limbs_fft_adjust
        // - offset != n in limbs_fft_neg
        // - zeros != n in limbs_fft_neg
        // - zeros == n in limbs_fft_neg
        // - trunc != n << 1 && trunc > n in limbs_fft_truncate1
        // - trunc != n << 1 && trunc <= n in limbs_fft_truncate1
        // - trunc == n << 1 in limbs_fft_truncate1
        // - trunc != n << 1 && trunc > n in limbs_ifft_truncate
        // - trunc != n << 1 && trunc > n in limbs_ifft_truncate1
        // - trunc != n << 1 && trunc <= n in limbs_ifft_truncate1
        // - trunc == n << 1 in limbs_ifft_truncate1
        test_big(&[10; 1000], &[10; 1000]);
        test_big(&[Limb::MAX; 1000], &[u64::MAX; 1000]);
        // - trunc > n << 1 in limbs_mul_mfa_truncate_sqrt2
        // - shift_bits < Limb::WIDTH in limbs_split_bits_worker
        // - i < n1 in limbs_fft_outer1_worker
        // - w.odd() in limbs_fft_outer1_worker
        // - j.even() in limbs_fft_outer1_worker
        // - i.even() in limbs_fft_outer1_worker
        // - n != 1 in limbs_fft_radix2_twiddle
        // - n == 1 in limbs_fft_radix2_twiddle
        // - b1 < nw in limbs_fft_butterfly_twiddle
        // - b2 < nw in limbs_fft_butterfly_twiddle
        // - j >= s in limbs_fft_outer1_worker;
        // - j < s in limbs_fft_outer1_worker
        // - j.odd() in limbs_fft_outer1_worker
        // - b1 < wn in limbs_fft_butterfly_sqrt2
        // - y != 0 in limbs_fft_butterfly_sqrt2
        // - limbs.even() in limbs_fft_butterfly_sqrt2
        // - !negate in limbs_fft_butterfly_sqrt2
        // - i.odd() in limbs_fft_outer1_worker
        // - b1 < wn in limbs_fft_adjust_sqrt2
        // - y != 0 first time in limbs_fft_adjust_sqrt2
        // - d != 0 in limbs_fft_mul_2expmod_2expp1
        // - y != 0 second time in limbs_fft_adjust_sqrt2
        // - limbs.even() in limbs_fft_adjust_sqrt2
        // - !negate in limbs_fft_adjust_sqrt2
        // - b1 >= wn in limbs_fft_adjust_sqrt2
        // - y == 0 first time in limbs_fft_adjust_sqrt2
        // - negate in limbs_fft_adjust_sqrt2
        // - x != 0 && y != 0 && x < y in limbs_butterfly_lsh_b
        // - offset == n in limbs_fft_neg_in_place
        // - offset != n in limbs_fft_neg_in_place
        // - zeros == n in limbs_fft_neg_in_place
        // - zeros != n in limbs_fft_neg_in_place
        // - d == 0 in limbs_fft_mul_2expmod_2expp1
        // - b2 >= nw in limbs_fft_butterfly_twiddle
        // - x != 0 && y != 0 && x > y in limbs_butterfly_lsh_b
        // - x != 0 && y == 0 in limbs_butterfly_lsh_b
        // - i >= n1 in limbs_fft_outer1_worker
        // - i < n1 in limbs_fft_outer2_worker
        // - trunc != n << 1 && trunc <= n in limbs_fft_truncate1_twiddle
        // - trunc == n << 1 in limbs_fft_truncate1_twiddle
        // - j >= s in limbs_fft_outer2_worker
        // - j < s in limbs_fft_outer2_worker
        // - i >= n1 in limbs_fft_outer2_worker
        // - s < trunc in limbs_fft_inner1_worker
        // - c.even() && c & 2 == 0 in limbs_fft_mulmod_2expp1
        // - limbs <= FFT_MULMOD_2EXPP1_CUTOFF in limbs_fft_mulmod_2expp1
        // - s >= trunc in limbs_fft_inner1_worker
        // - i < n2 in limbs_fft_inner2_worker
        // - i >= n2 in limbs_fft_inner2_worker
        // - i < n1 in limbs_ifft_outer1_worker
        // - j >= s in limbs_ifft_outer1_worker
        // - j < s in limbs_ifft_outer1_worker
        // - n != 1 in limbs_ifft_radix2_twiddle
        // - n == 1 in limbs_ifft_radix2_twiddle
        // - b1 < nw in limbs_ifft_butterfly_twiddle
        // - b2 < nw in limbs_ifft_butterfly_twiddle
        // - !negate1 in limbs_ifft_butterfly_twiddle
        // - !negate2 in limbs_ifft_butterfly_twiddle
        // - x != 0 && y != 0 && x < y in limbs_butterfly_rsh_b
        // - b2 >= nw in limbs_ifft_butterfly_twiddle
        // - negate2 in limbs_ifft_butterfly_twiddle
        // - x != 0 && y != 0 && x > y in limbs_butterfly_rsh_b
        // - x != 0 && y == 0 in limbs_butterfly_rsh_b
        // - i >= n1 in limbs_ifft_outer1_worker
        // - i < n1 in limbs_ifft_outer2_worker
        // - j >= s in limbs_ifft_outer2_worker
        // - j < s in limbs_ifft_outer2_worker
        // - w.odd() first time in limbs_ifft_outer2_worker
        // - i.even() in limbs_ifft_outer2_worker
        // - trunc != n << 1 && trunc <= n in limbs_ifft_truncate1_twiddle
        // - trunc == n << 1 in limbs_ifft_truncate1_twiddle
        // - w.odd() second time in limbs_ifft_outer2_worker
        // - j.even() in limbs_ifft_outer2_worker
        // - i.odd() in limbs_ifft_outer2_worker
        // - offset == n in limbs_fft_neg
        // - j.odd() in limbs_ifft_outer2_worker
        // - b1 >= wn in limbs_ifft_butterfly_sqrt2
        // - b1 != 0 in limbs_ifft_butterfly_sqrt2
        // - y != 0 in limbs_ifft_butterfly_sqrt2
        // - limbs.even() in limbs_ifft_butterfly_sqrt2
        // - !negate in limbs_ifft_butterfly_sqrt2
        // - xs.is_empty() in limbs_fft_sumdiff
        // - b1 < wn in limbs_ifft_butterfly_sqrt2
        // - negate in limbs_ifft_butterfly_sqrt2
        // - b1 == 0 in limbs_ifft_butterfly_sqrt2
        // - i >= n1 in limbs_ifft_outer2_worker
        // - shift_bits < Limb::WIDTH second time in limbs_fft_combine_bits
        // - j1 + j2 - 1 <= 3 * n
        test_big(&[10; 69569], &[10; 2591]);
        // - trunc != n << 1 && trunc > n in limbs_fft_truncate1_twiddle
        // - trunc != n << 1 && trunc > n in limbs_ifft_truncate1_twiddle
        test_big(&[10; 44636], &[10; 25927]);
        // - w.even() in limbs_fft_outer1_worker
        // - w.even() first time in limbs_ifft_outer2_worker
        // - w.even() second time in limbs_ifft_outer2_worker
        // - 50 cutoff: limbs > cutoff in limbs_fft_mulmod_2expp1
        // - 50 cutoff: depth >= 12 in limbs_fft_mulmod_2expp1
        // - 50 cutoff: top_bits == 0 in limbs_fft_split_bits
        // - 50 cutoff: i < num in limbs_split_limbs_worke
        // - 50 cutoff: i >= num in limbs_split_limbs_worker
        // - 50 cutoff: i >= length in limbs_fft_split_limbs
        // - 50 cutoff: total_limbs <= skip in limbs_fft_split_limbs
        // - 50 cutoff: w.even() in limbs_fft_negacyclic
        // - 50 cutoff: top_bits == 0 in limbs_fft_combine_bits
        // - 50 cutoff: r[j] == 0 && SignedLimb::wrapping_from(xss[j][limbs]) < 0 first time in
        //   limbs_fft_mulmod_2expp1_helper
        // - 50 cutoff: r[j] == 0 && SignedLimb::wrapping_from(xss[j][limbs]) >= 0 in
        //   limbs_fft_mulmod_2expp1_helper
        // - 50 cutoff: r[j] != 0 && SignedLimb::wrapping_from(xss[j][limbs]) >= 0 in
        //   limbs_fft_mulmod_2expp1_helper
        // - 50 cutoff: limb_add != 0 in limbs_fft_mulmod_2expp1_helper
        // - 50 cutoff: r[j] == 0 && SignedLimb::wrapping_from(xss[j][limbs]) < 0 second time in
        //   limbs_fft_mulmod_2expp1_helper
        test_big(&[10; 178940], &[10; 21015]);
        // - w.odd() in limbs_fft_truncate_sqrt2
        // - b1 >= wn in limbs_fft_butterfly_sqrt2
        // - negate in limbs_fft_butterfly_sqrt2
        // - w.odd() in limbs_ifft_truncate_sqrt2
        test_big(&[10; 10758], &[10; 14125]);
        // - depth >= 6
        test_big(&[10; 10000], &[10; 10000]);
        // - w == 1
        // - w != 1
        // - depth >= 11
        // - j1 + j2 - 1 > 3 * n
        test_big(&[10; 100000], &[10; 100000]);
        // - limbs.odd() in limbs_fft_butterfly_sqrt2
        // - limbs.odd() in limbs_fft_adjust_sqrt2
        // - limbs.odd() in limbs_ifft_butterfly_sqrt2
        test_big(&[10; 188], &[10; 3231]);
        test_big(&[Limb::MAX; 27300], &[Limb::MAX; 49384]);
        let xs = &[
            (0, 3772),
            (18446743936270598144, 1),
            (u64::MAX, 12730),
            (9223372036854775807, 1),
            (0, 10796),
        ];
        let ys = &[
            (0, 1181),
            (18446744073709486080, 1),
            (u64::MAX, 2874),
            (1048575, 1),
            (0, 14962),
            (18410715276690587648, 1),
            (u64::MAX, 10211),
            (4611686018427387903, 1),
            (0, 111),
            (18410715276690587648, 1),
            (u64::MAX, 1566),
            (9007199254740991, 1),
            (0, 468),
            (18442240474082181120, 1),
            (u64::MAX, 3062),
            (35184372088831, 1),
            (0, 830),
            (18446181123756130304, 1),
            (u64::MAX, 11736),
            (63, 1),
            (0, 2373),
        ];
        test_big(&rle_decode(xs), &rle_decode(ys));
    }
    #[cfg(feature = "32_bit_limbs")]
    {
        // - shift_bits == 0 in limbs_fft_split_bits
        test(&[0; 49], &[0; 64], &[10; 113], &[0; 113]);
        // - t[limbs] == Limb::MAX in limbs_fft_normmod_2expp1
        // - cy == 0 && cz != 0 in limbs_fft_mulmod_2expp1_basecase_same
        test(
            &[0; 49],
            &[
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 1,
            ],
            &[10; 113],
            &[0; 113],
        );
        // - shift_bits == 0 second time in limbs_fft_combine_bits
        test_big(&[10; 8284], &[10; 23341]);
        // - x == 0 in limbs_fft_adjust
        test_big(&[10; 17027], &[10; 15816]);
        // - trunc <= n << 1 in limbs_mul_truncate_sqrt2
        test_big(&[10; 32127], &[10; 32870]);
        // - 50 cutoff: depth < 12 in limbs_fft_mulmod_2expp1
        test_big(&[10; 178940], &[10; 21015]);
    }
}

fn verify_limbs_square_to_out_fft(out_before: &[Limb], xs: &[Limb], out_after: &[Limb]) {
    let n = Natural::from_limbs_asc(xs).square();
    let mut ns = n.into_limbs_asc();
    let len = xs.len() << 1;
    ns.resize(len, 0);
    assert_eq!(ns, &out_after[..len]);
    assert_eq!(&out_after[len..], &out_before[len..]);
}

#[test]
fn test_limbs_square_to_out_fft() {
    #[cfg(not(feature = "32_bit_limbs"))]
    let test = |xs: &[Limb], out_before: &[Limb], out_after: &[Limb]| {
        let mut out = out_before.to_vec();
        let mut scratch = vec![0; limbs_square_to_out_fft_scratch_len(xs.len())];
        limbs_square_to_out_fft(&mut out, xs, &mut scratch);
        assert_eq!(out, out_after);
        let mut out = out_before.to_vec();
        let mut scratch = vec![0; limbs_square_to_out_fft_with_cutoff_scratch_len(xs.len(), 50)];
        limbs_square_to_out_fft_with_cutoff(&mut out, xs, 50, &mut scratch);
        assert_eq!(out, out_after);

        verify_limbs_square_to_out_fft(out_before, xs, out_after);
    };
    let test_big = |xs: &[Limb]| {
        let mut out = vec![0; xs.len() << 1];
        let out_before = out.clone();
        let mut scratch = vec![0; limbs_square_to_out_fft_scratch_len(xs.len())];
        limbs_square_to_out_fft(&mut out, xs, &mut scratch);
        let out_after = out;
        let mut out = out_before.clone();
        let mut scratch = vec![0; limbs_square_to_out_fft_with_cutoff_scratch_len(xs.len(), 50)];
        limbs_square_to_out_fft_with_cutoff(&mut out, xs, 50, &mut scratch);
        assert_eq!(out, out_after);

        verify_limbs_square_to_out_fft(&out_before, xs, &out);
    };
    #[cfg(not(feature = "32_bit_limbs"))]
    {
        // - depth < 11
        // - depth < 6
        // - w > wadj
        // - (j1 << 1) - 1 <= n << 2 || w <= wadj
        // - (j1 << 1) - 1 > n << 2 && w > wadj
        // - trunc > n << 1 in limbs_mul_truncate_sqrt2_same
        // - cy == 0 in limbs_fft_mulmod_2expp1_basecase_same2
        // - cy == 0 && cz == 0 in limbs_fft_mulmod_2expp1_basecase_same2
        // - k == 0 in limbs_fft_mulmod_2expp1_internal_same2
        test(
            &[
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                1,
            ],
            &[10; 58],
            &[
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                1, 0,
            ],
        );
        // - w == 1
        test_big(&[10; 100]);
        // - w != 1
        // - depth >= 6
        test_big(&[10; 1000]);
        // - depth >= 11
        // - (j1 << 1) - 1 > 3 * n
        // - trunc > n << 1 in limbs_mul_mfa_truncate_sqrt2_same
        // - s < trunc in limbs_fft_inner1_worker_same
        // - c.even() && c & 2 == 0 in limbs_fft_mulmod_2expp1_same
        // - limbs <= FFT_MULMOD_2EXPP1_CUTOFF in limbs_fft_mulmod_2expp1_same
        // - s >= trunc in limbs_fft_inner1_worker_same
        // - i < n2 in limbs_fft_inner2_worker_same
        // - i >= n2 in limbs_fft_inner2_worker_same
        // - 50 cutoff: limbs > cutoff in limbs_fft_mulmod_2expp1_same
        // - 50 cutoff: depth >= 12 in limbs_fft_mulmod_2expp1_same
        // - 50 cutoff: r[j] == 0 && SignedLimb::wrapping_from(xss[j][limbs]) < 0 first time in
        //   limbs_fft_mulmod_2expp1_helper_same
        // - 50 cutoff: r[j] == 0 && SignedLimb::wrapping_from(xss[j][limbs]) >= 0 in
        //   limbs_fft_mulmod_2expp1_helper_same
        // - 50 cutoff: r[j] != 0 && SignedLimb::wrapping_from(xss[j][limbs]) >= 0 in
        //   limbs_fft_mulmod_2expp1_helper_same
        // - 50 cutoff: limb_add != 0 in limbs_fft_mulmod_2expp1_helper_same
        // - 50 cutoff: r[j] == 0 && SignedLimb::wrapping_from(xss[j][limbs]) < 0 second time in
        //   limbs_fft_mulmod_2expp1_helper_same
        test_big(&[10; 100000]);
        // - (j1 << 1) - 1 <= 3 * n
        test_big(&[10; 38424]);
        test_big(&[10; 5354]);
        let xs = rle_decode(&[
            (0, 9249),
            (18410715276690587648, 1),
            (u64::MAX, 11159),
            (134217727, 1),
            (0, 2780),
            (18446726481523507200, 1),
            (u64::MAX, 1972),
            (8796093022207, 1),
            (0, 2972),
            (18446742974197923840, 1),
            (u64::MAX, 1456),
            (255, 1),
            (0, 1446),
            (16140901064495857664, 1),
            (u64::MAX, 534),
            (2047, 1),
            (0, 4901),
            (18446603336221196288, 1),
            (u64::MAX, 1262),
            (2251799813685247, 1),
            (0, 802),
            (18446726481523507200, 1),
            (u64::MAX, 131),
            (137438953471, 1),
            (0, 1394),
            (18446744073709551614, 1),
            (u64::MAX, 3007),
            (8589934591, 1),
            (0, 960),
            (18446726481523507200, 1),
            (u64::MAX, 2902),
            (288230376151711743, 1),
            (0, 7719),
        ]);
        test_big(&xs);
    }
    #[cfg(feature = "32_bit_limbs")]
    {
        // - w <= wadj
        test_big(&[10; 25186]);
        // - trunc <= n << 1 in limbs_mul_truncate_sqrt2_same
        test_big(&[10; 32562]);
        // - depth < 12 in limbs_fft_mulmod_2expp1_same
        test_big(&[10; 103030]);
    }
}

#[test]
fn limbs_mul_greater_to_out_fft_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32768);
    config.insert("mean_stripe_n", 4096 << Limb::LOG_WIDTH);
    unsigned_vec_triple_gen_var_60().test_properties_with_config(
        &config,
        |(out_before, xs, ys)| {
            let mut out = out_before.to_vec();
            let mut scratch = vec![0; limbs_mul_greater_to_out_fft_scratch_len(xs.len(), ys.len())];
            limbs_mul_greater_to_out_fft(&mut out, &xs, &ys, &mut scratch);
            verify_limbs_mul_greater_to_out_fft(&out_before, &xs, &ys, &out);
            let out_after = out;
            let mut out = out_before.to_vec();
            let mut scratch =
                vec![
                    0;
                    limbs_mul_greater_to_out_fft_with_cutoff_scratch_len(xs.len(), ys.len(), 50)
                ];
            limbs_mul_greater_to_out_fft_with_cutoff(&mut out, &xs, &ys, 50, &mut scratch);
            assert_eq!(out, out_after);
        },
    );
}

#[test]
fn limbs_square_to_out_fft_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32768);
    config.insert("mean_stripe_n", 4096 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen_var_33().test_properties_with_config(&config, |(out_before, xs)| {
        let mut out = out_before.to_vec();
        let mut scratch = vec![0; limbs_square_to_out_fft_scratch_len(xs.len())];
        limbs_square_to_out_fft(&mut out, &xs, &mut scratch);
        verify_limbs_square_to_out_fft(&out_before, &xs, &out);
        let out_after = out;
        let mut out = out_before.to_vec();
        let mut scratch = vec![0; limbs_square_to_out_fft_with_cutoff_scratch_len(xs.len(), 50)];
        limbs_square_to_out_fft_with_cutoff(&mut out, &xs, 50, &mut scratch);
        assert_eq!(out, out_after);
    });
}

#[test]
fn limbs_mul_low_same_length_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 512);
    config.insert("mean_stripe_n", 256 << Limb::LOG_WIDTH);
    unsigned_vec_triple_gen_var_1().test_properties_with_config(&config, |(out_before, xs, ys)| {
        let mut out = out_before.to_vec();
        limbs_mul_low_same_length(&mut out, &xs, &ys);
        verify_mul_low_1(&out_before, &xs, &ys, &out);
    });
}

#[test]
fn test_mul() {
    let test = |s, t, out| {
        let u = Natural::from_str(s).unwrap();
        let v = Natural::from_str(t).unwrap();

        let mut n = u.clone();
        n *= v.clone();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = u.clone();
        n *= &v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone() * v.clone();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &u * v.clone();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone() * &v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &u * &v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = BigUint::from_str(s).unwrap() * BigUint::from_str(t).unwrap();
        assert_eq!(n.to_string(), out);

        let n = rug::Integer::from_str(s).unwrap() * rug::Integer::from_str(t).unwrap();
        assert_eq!(n.to_string(), out);
    };
    test("0", "0", "0");
    test("0", "123", "0");
    test("123", "0", "0");
    test("1", "123", "123");
    test("123", "1", "123");
    test("123", "456", "56088");
    test("0", "1000000000000", "0");
    test("1000000000000", "0", "0");
    test("1", "1000000000000", "1000000000000");
    test("1000000000000", "1", "1000000000000");
    test("1000000000000", "123", "123000000000000");
    test("123", "1000000000000", "123000000000000");
    test("123456789000", "987654321000", "121932631112635269000000");
    test("4294967295", "2", "8589934590");
    test("18446744073709551615", "2", "36893488147419103230");
    test("4294967295", "4294967295", "18446744065119617025");
    test(
        "147502279655636565600250358452694051893980186696958535174009956523855720107322638159749368\
        0808217479494744305876890972595771484769733857514529616096199394092858302265998260483416016\
        5763904522044264005938281072568140883513713255548643044250086110483617215935636533809248102\
        6926590789142079805638445494760177551776636747830014495012489743990407355232286842071418922\
        9921358409573480901624487977319782755422730834468673438076805532952821406024399006814390166\
        6949827530796971086011267864607814906313334525518102221919643040440267323688341889035864376\
        1377246644579088153222669672271414315240318439843720039808993886410874969340996645010795670\
        2133518716987668865936529827437388042190084309005369564717390726257594902619365180097509576\
        6240189037770619308206906414128686856349950952623970023039440323701643457411485666776354448\
        186307133288106956593939073729500658176632828099789",
        "577397114388109712462006371470162814529304445639807296878809567953200969820156259914159240\
        9106139481288193067515284601342023565222679498917484131095648263181800618990427694244342686\
        4412105186059052689237237088193855584354278755933606296018800151986520872701706693002473648\
        4330061421236425747083307907706860804054565348593527605104495080560663025897787060638154303\
        7631781316565710346299551930891169154491973589315700505458672804104869879731391323700304",
        "851673906388325341550957943071111911557800036845129556099360938813259608650267079456739978\
        1156959952275409185911771336067392377245918291754269000751186715279414560474882570499082990\
        4913122978897463970860833616251189242098804876664368441608727895141238953179204529256780277\
        5978105200286025161944212712977056343127682601975191673217459602567633602198262568921008081\
        9448556670912575287371251190800855926311768876808375177446530243635212748346921654224589861\
        0625170426812525829689862407515510419445335472631905610235915226032848323874067128872385291\
        3730739275467227364692195226129501338887049710586931141309357190341064532366013123280106098\
        6468151628797945455179649866890394481799639832540978091736379482964522229064478167730317490\
        8194108506704480750395054067032502530392147690725919399930683143510771646869931527123340650\
        0547649792331568913460415939722111305270588701531404490040034302102101083691706550376288655\
        2667382899390792494118931379237432071316543313379792218794371176529684614085109418328963817\
        0601432767270419229719490809539776535671938041618536196941370647945336401901450921413823163\
        4059991707077834107830876756821880651429748186401020760113859498185638133726165286481741014\
        9079906337286599226335508424466369316294442004040440528589582239717042654541745348050157252\
        3448224036804997350851153108395928780441635856",
    );
    let large_power_of_2 = Natural::power_of_2(100_000) * Natural::power_of_2(100_000);
    assert!(large_power_of_2.is_valid());
    assert_eq!(large_power_of_2.checked_log_base_2(), Some(200_000));
}

#[test]
fn test_multiplying_large_powers_of_2() {
    for i in 0..400_000 {
        let p = Natural::power_of_2(i) * Natural::power_of_2(i);
        assert!(p.is_valid());
        assert_eq!(p.checked_log_base_2(), Some(i << 1));
    }
}

#[test]
fn limbs_product_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 64);
    config.insert("mean_stripe_n", 16);
    unsigned_vec_gen_var_6().test_properties_with_config(&config, |mut xs| {
        let xs_old = xs.clone();
        let mut out = vec![0; xs.len()];
        let out_len = limbs_product(&mut out, &mut xs);
        out.truncate(out_len);
        let xs = xs_old;
        let mut out_alt = vec![0; xs.len()];
        let out_len = limbs_product_naive(&mut out_alt, &xs);
        out_alt.truncate(out_len);
        assert_eq!(
            Natural::from_owned_limbs_asc(out),
            Natural::from_owned_limbs_asc(out_alt)
        );
    });
}

#[test]
fn mul_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_bits_n", 2048);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    natural_pair_gen().test_properties_with_config(&config, |(x, y)| {
        let product_val_val = x.clone() * y.clone();
        let product_val_ref = x.clone() * &y;
        let product_ref_val = &x * y.clone();
        let product = &x * &y;
        assert!(product_val_val.is_valid());
        assert!(product_val_ref.is_valid());
        assert!(product_ref_val.is_valid());
        assert!(product.is_valid());
        assert_eq!(product_val_val, product);
        assert_eq!(product_val_ref, product);
        assert_eq!(product_ref_val, product);

        let mut mut_x = x.clone();
        mut_x *= y.clone();
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, product);

        let mut mut_x = x.clone();
        mut_x *= &y;
        assert_eq!(mut_x, product);
        assert!(mut_x.is_valid());

        assert_eq!(
            Natural::from(&(BigUint::from(&x) * BigUint::from(&y))),
            product
        );
        assert_eq!(
            Natural::exact_from(&(rug::Integer::from(&x) * rug::Integer::from(&y))),
            product
        );
        assert_eq!(&y * &x, product);
        if x != 0 {
            let (q, r) = (&product).div_mod(&x);
            assert_eq!(q, y);
            assert_eq!(r, 0);
        }
        if y != 0 {
            let (q, r) = (&product).div_mod(&y);
            assert_eq!(q, x);
            assert_eq!(r, 0);
        }
        if x != 0 && y != 0 {
            assert!(product >= x);
            assert!(product >= y);
        }
    });

    unsigned_pair_gen_var_27::<Limb>().test_properties(|(x, y)| {
        assert_eq!(
            Natural::from(DoubleLimb::from(x) * DoubleLimb::from(y)),
            Natural::from(x) * Natural::from(y)
        );
    });

    natural_gen().test_properties(|ref x| {
        assert_eq!(x * Natural::ZERO, 0);
        assert_eq!(Natural::ZERO * x, 0);
        assert_eq!(x * Natural::ONE, *x);
        assert_eq!(Natural::ONE * x, *x);
        assert_eq!(x * x, x.square());
    });

    natural_triple_gen().test_properties(|(ref x, ref y, ref z)| {
        assert_eq!((x * y) * z, x * (y * z));
        assert_eq!(x * (y + z), x * y + x * z);
        assert_eq!((x + y) * z, x * z + y * z);
    });
}

#[test]
fn product_properties() {
    natural_vec_gen().test_properties(|xs| {
        let product = Natural::product(xs.iter().cloned());
        assert!(product.is_valid());

        let product_alt = Natural::product(xs.iter());
        assert!(product_alt.is_valid());
        assert_eq!(product_alt, product);

        let product_alt = natural_product_naive(xs.into_iter());
        assert!(product_alt.is_valid());
        assert_eq!(product_alt, product);
    });

    natural_gen().test_properties(|x| {
        assert_eq!(Natural::product(once(&x)), x);
        assert_eq!(Natural::product(once(x.clone())), x);
    });

    natural_pair_gen().test_properties(|(x, y)| {
        let product = &x * &y;
        assert_eq!(Natural::product([&x, &y].into_iter()), product);
        assert_eq!(Natural::product([x, y].into_iter()), product);
    });
}
