// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{ModPowerOf2, ModPowerOf2Neg, PowerOf2};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::slices::slice_test_zero;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{
    large_type_gen_var_9, unsigned_pair_gen_var_7, unsigned_vec_gen, unsigned_vec_pair_gen_var_31,
    unsigned_vec_pair_gen_var_6, unsigned_vec_triple_gen_var_31, unsigned_vec_triple_gen_var_40,
    unsigned_vec_unsigned_pair_gen, unsigned_vec_unsigned_pair_gen_var_1,
    unsigned_vec_unsigned_vec_bool_triple_gen_var_1,
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_1,
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_24,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::arithmetic::sub::{
    limbs_slice_sub_in_place_right, limbs_sub, limbs_sub_greater_in_place_left,
    limbs_sub_greater_to_out, limbs_sub_limb, limbs_sub_limb_in_place, limbs_sub_limb_to_out,
    limbs_sub_same_length_in_place_left, limbs_sub_same_length_in_place_right,
    limbs_sub_same_length_in_place_with_overlap, limbs_sub_same_length_to_out,
    limbs_sub_same_length_to_out_with_overlap, limbs_sub_same_length_with_borrow_in_in_place_left,
    limbs_sub_same_length_with_borrow_in_to_out, limbs_vec_sub_in_place_right,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{natural_gen, natural_pair_gen_var_10};
use malachite_nz::test_util::natural::arithmetic::sub::{
    limbs_sub_same_length_in_place_with_overlap_naive,
    limbs_sub_same_length_to_out_with_overlap_naive,
};
use num::BigUint;
use rug;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_sub_limb() {
    let test = |xs: &[Limb], y: Limb, out: (Vec<Limb>, bool)| {
        assert_eq!(limbs_sub_limb(xs, y), out);
    };
    test(&[], 0, (vec![], false));
    test(&[1], 2, (vec![u32::MAX], true));
    test(&[6, 7], 2, (vec![4, 7], false));
    test(&[100, 101, 102], 10, (vec![90, 101, 102], false));
    test(&[123, 456], 78, (vec![45, 456], false));
    test(&[123, 456], 789, (vec![4294966630, 455], false));
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_sub_limb_to_out() {
    let test = |out_before: &[Limb], xs: &[Limb], y: Limb, borrow: bool, out_after: &[Limb]| {
        let mut out = out_before.to_vec();
        assert_eq!(limbs_sub_limb_to_out(&mut out, xs, y), borrow);
        assert_eq!(out, out_after);
    };
    test(&[10, 10, 10, 10], &[], 0, false, &[10, 10, 10, 10]);
    test(&[10, 10, 10, 10], &[1], 2, true, &[u32::MAX, 10, 10, 10]);
    test(&[10, 10, 10, 10], &[6, 7], 2, false, &[4, 7, 10, 10]);
    test(
        &[10, 10, 10, 10],
        &[100, 101, 102],
        10,
        false,
        &[90, 101, 102, 10],
    );
    test(
        &[10, 10, 10, 10],
        &[123, 456],
        78,
        false,
        &[45, 456, 10, 10],
    );
    test(
        &[10, 10, 10, 10],
        &[123, 456],
        789,
        false,
        &[4294966630, 455, 10, 10],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_limb_to_out_fail() {
    limbs_sub_limb_to_out(&mut [10], &[10, 10], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_sub_limb_in_place() {
    let test = |xs: &[Limb], y: Limb, borrow: bool, out: &[Limb]| {
        let mut xs = xs.to_vec();
        assert_eq!(limbs_sub_limb_in_place(&mut xs, y), borrow);
        assert_eq!(xs, out);
    };
    test(&[], 0, false, &[]);
    test(&[1], 2, true, &[u32::MAX]);
    test(&[6, 7], 2, false, &[4, 7]);
    test(&[100, 101, 102], 10, false, &[90, 101, 102]);
    test(&[123, 456], 78, false, &[45, 456]);
    test(&[123, 456], 789, false, &[4294966630, 455]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_sub() {
    let test = |xs, ys, out| {
        assert_eq!(limbs_sub(xs, ys), out);
    };
    test(&[], &[], (vec![], false));
    test(&[2], &[], (vec![2], false));
    test(&[3], &[2], (vec![1], false));
    test(&[2], &[3], (vec![u32::MAX], true));
    test(&[1, 2, 3], &[1, 1, 1], (vec![0, 1, 2], false));
    test(
        &[1, 1, 1],
        &[1, 2, 3],
        (vec![0, u32::MAX, 0xfffffffd], true),
    );
    test(
        &[1, 2, 3],
        &[6, 7],
        (vec![0xfffffffb, 0xfffffffa, 2], false),
    );
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        (vec![u32::MAX - 1, u32::MAX, 1], false),
    );
    test(
        &[102, 101, 100],
        &[100, 101, 102],
        (vec![2, 0, u32::MAX - 1], true),
    );
    test(&[0, 0, 0], &[1], (vec![u32::MAX, u32::MAX, u32::MAX], true));
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_fail() {
    limbs_sub(&[6, 7], &[1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_sub_same_length_to_out() {
    let test = |xs, ys, out_before: &[Limb], borrow, out_after| {
        let mut out = out_before.to_vec();
        assert_eq!(limbs_sub_same_length_to_out(&mut out, xs, ys), borrow);
        assert_eq!(out, out_after);
    };
    test(&[], &[], &[0, 0], false, vec![0, 0]);
    test(&[3], &[2], &[0, 0], false, vec![1, 0]);
    test(&[2], &[3], &[0, 0], true, vec![u32::MAX, 0]);
    test(
        &[1, 2, 3],
        &[1, 1, 1],
        &[0, 1, 2, 5],
        false,
        vec![0, 1, 2, 5],
    );
    test(
        &[1, 1, 1],
        &[1, 2, 3],
        &[5, 5, 5, 5],
        true,
        vec![0, u32::MAX, 0xfffffffd, 5],
    );
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        &[10, 10, 10, 10],
        false,
        vec![u32::MAX - 1, u32::MAX, 1, 10],
    );
    test(
        &[102, 101, 100],
        &[100, 101, 102],
        &[10, 10, 10, 10],
        true,
        vec![2, 0, u32::MAX - 1, 10],
    );
    test(
        &[0, 0, 0],
        &[1, 0, 0],
        &[10, 10, 10, 10],
        true,
        vec![u32::MAX, u32::MAX, u32::MAX, 10],
    );
    test(
        &[u32::MAX, u32::MAX, 0xffffff, 0, 0],
        &[0, 0, 0, 4294967232, u32::MAX],
        &[10, 10, 10, 10, 10, 10],
        true,
        vec![u32::MAX, u32::MAX, 0xffffff, 64, 0, 10],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_same_length_to_out_fail_1() {
    let mut out = vec![10, 10];
    limbs_sub_same_length_to_out(&mut out, &[6, 7, 8], &[1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_same_length_to_out_fail_2() {
    let mut out = vec![10, 10, 10];
    limbs_sub_same_length_to_out(&mut out, &[6, 7, 8], &[1, 2]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_sub_greater_to_out() {
    let test = |xs, ys, out_before: &[Limb], borrow, out_after| {
        let mut out = out_before.to_vec();
        assert_eq!(limbs_sub_greater_to_out(&mut out, xs, ys), borrow);
        assert_eq!(out, out_after);
    };
    test(&[], &[], &[0, 0], false, vec![0, 0]);
    test(&[2], &[], &[0, 0], false, vec![2, 0]);
    test(&[3], &[2], &[0, 0], false, vec![1, 0]);
    test(&[2], &[3], &[0, 0], true, vec![u32::MAX, 0]);
    test(
        &[1, 2, 3],
        &[1, 1, 1],
        &[0, 1, 2, 5],
        false,
        vec![0, 1, 2, 5],
    );
    test(
        &[1, 1, 1],
        &[1, 2, 3],
        &[5, 5, 5, 5],
        true,
        vec![0, u32::MAX, 0xfffffffd, 5],
    );
    test(
        &[1, 2, 3],
        &[6, 7],
        &[0, 0, 0],
        false,
        vec![0xfffffffb, 0xfffffffa, 2],
    );
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        &[10, 10, 10, 10],
        false,
        vec![u32::MAX - 1, u32::MAX, 1, 10],
    );
    test(
        &[102, 101, 100],
        &[100, 101, 102],
        &[10, 10, 10, 10],
        true,
        vec![2, 0, u32::MAX - 1, 10],
    );
    test(
        &[0, 0, 0],
        &[1],
        &[10, 10, 10, 10],
        true,
        vec![u32::MAX, u32::MAX, u32::MAX, 10],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_greater_to_out_fail_1() {
    let mut out = vec![10, 10];
    limbs_sub_greater_to_out(&mut out, &[6, 7, 8], &[1, 2]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_greater_to_out_fail_2() {
    let mut out = vec![10, 10, 10];
    limbs_sub_greater_to_out(&mut out, &[6, 7], &[1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_sub_same_length_in_place_left() {
    let test = |xs_before: &[Limb], ys, borrow, xs_after| {
        let mut xs = xs_before.to_vec();
        assert_eq!(limbs_sub_same_length_in_place_left(&mut xs, ys), borrow);
        assert_eq!(xs, xs_after);
    };
    test(&[], &[], false, vec![]);
    test(&[3], &[2], false, vec![1]);
    test(&[2], &[3], true, vec![u32::MAX]);
    test(&[1, 2, 3], &[1, 1, 1], false, vec![0, 1, 2]);
    test(&[1, 1, 1], &[1, 2, 3], true, vec![0, u32::MAX, 0xfffffffd]);
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        false,
        vec![u32::MAX - 1, u32::MAX, 1],
    );
    test(
        &[102, 101, 100],
        &[100, 101, 102],
        true,
        vec![2, 0, u32::MAX - 1],
    );
    test(
        &[0, 0, 0],
        &[1, 0, 0],
        true,
        vec![u32::MAX, u32::MAX, u32::MAX],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_same_length_in_place_left_fail() {
    limbs_sub_same_length_in_place_left(&mut [6, 7], &[1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_sub_greater_in_place_left() {
    let test = |xs_before: &[Limb], ys, borrow, xs_after| {
        let mut xs = xs_before.to_vec();
        assert_eq!(limbs_sub_greater_in_place_left(&mut xs, ys), borrow);
        assert_eq!(xs, xs_after);
    };
    test(&[], &[], false, vec![]);
    test(&[2], &[], false, vec![2]);
    test(&[3], &[2], false, vec![1]);
    test(&[2], &[3], true, vec![u32::MAX]);
    test(&[1, 2, 3], &[1, 1, 1], false, vec![0, 1, 2]);
    test(&[1, 1, 1], &[1, 2, 3], true, vec![0, u32::MAX, 0xfffffffd]);
    test(&[1, 2, 3], &[6, 7], false, vec![0xfffffffb, 0xfffffffa, 2]);
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        false,
        vec![u32::MAX - 1, u32::MAX, 1],
    );
    test(
        &[102, 101, 100],
        &[100, 101, 102],
        true,
        vec![2, 0, u32::MAX - 1],
    );
    test(&[0, 0, 0], &[1], true, vec![u32::MAX, u32::MAX, u32::MAX]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_greater_in_place_left_fail() {
    limbs_sub_greater_in_place_left(&mut [6, 7], &[1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_sub_same_length_in_place_right() {
    let test = |xs, ys_before: &[Limb], borrow, ys_after| {
        let mut ys = ys_before.to_vec();
        assert_eq!(limbs_sub_same_length_in_place_right(xs, &mut ys), borrow);
        assert_eq!(ys, ys_after);
    };
    test(&[], &[], false, vec![]);
    test(&[3], &[2], false, vec![1]);
    test(&[2], &[3], true, vec![u32::MAX]);
    test(&[1, 2, 3], &[1, 1, 1], false, vec![0, 1, 2]);
    test(&[1, 1, 1], &[1, 2, 3], true, vec![0, u32::MAX, 0xfffffffd]);
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        false,
        vec![u32::MAX - 1, u32::MAX, 1],
    );
    test(
        &[102, 101, 100],
        &[100, 101, 102],
        true,
        vec![2, 0, u32::MAX - 1],
    );
    test(
        &[0, 0, 0],
        &[1, 0, 0],
        true,
        vec![u32::MAX, u32::MAX, u32::MAX],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_same_length_in_place_right_fail_1() {
    limbs_sub_same_length_in_place_right(&[6, 7], &mut [1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_same_length_in_place_right_fail_2() {
    limbs_sub_same_length_in_place_right(&[1, 2], &mut [6, 7, 8]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_slice_sub_in_place_right() {
    let test = |xs: &[Limb], ys_before: &[Limb], len: usize, borrow: bool, ys_after: &[Limb]| {
        let mut ys = ys_before.to_vec();
        assert_eq!(limbs_slice_sub_in_place_right(xs, &mut ys, len), borrow);
        assert_eq!(ys, ys_after);
    };
    test(&[], &[], 0, false, &[]);
    test(&[123, 456], &[789, 123], 2, false, &[4294966630, 332]);
    test(&[123, 456], &[789, 123], 1, false, &[4294966630, 455]);
    test(&[123, 0], &[789, 123], 1, true, &[4294966630, u32::MAX]);
    test(&[123, 456], &[789, 123], 0, false, &[123, 456]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_slice_sub_in_place_right_fail_1() {
    limbs_slice_sub_in_place_right(&[6, 7], &mut [1, 2, 3], 1);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_slice_sub_in_place_right_fail_2() {
    limbs_slice_sub_in_place_right(&[6, 7], &mut [1, 2], 3);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_vec_sub_in_place_right() {
    let test = |xs, ys_before: &[Limb], borrow, ys_after| {
        let mut ys = ys_before.to_vec();
        assert_eq!(limbs_vec_sub_in_place_right(xs, &mut ys), borrow);
        assert_eq!(ys, ys_after);
    };
    test(&[], &[], false, vec![]);
    test(&[2], &[], false, vec![2]);
    test(&[3], &[2], false, vec![1]);
    test(&[2], &[3], true, vec![u32::MAX]);
    test(&[1, 2, 3], &[1, 1, 1], false, vec![0, 1, 2]);
    test(&[1, 1, 1], &[1, 2, 3], true, vec![0, u32::MAX, 0xfffffffd]);
    test(&[1, 2, 3], &[6, 7], false, vec![0xfffffffb, 0xfffffffa, 2]);
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        false,
        vec![u32::MAX - 1, u32::MAX, 1],
    );
    test(
        &[102, 101, 100],
        &[100, 101, 102],
        true,
        vec![2, 0, u32::MAX - 1],
    );
    test(&[0, 0, 0], &[1], true, vec![u32::MAX, u32::MAX, u32::MAX]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_vec_sub_in_place_right_fail() {
    limbs_vec_sub_in_place_right(&[6, 7], &mut vec![1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_sub_same_length_with_borrow_in_to_out() {
    let test = |xs, ys, borrow_in, out_before: &[Limb], borrow, out_after| {
        let mut out = out_before.to_vec();
        assert_eq!(
            limbs_sub_same_length_with_borrow_in_to_out(&mut out, xs, ys, borrow_in),
            borrow
        );
        assert_eq!(out, out_after);
    };
    test(&[], &[], false, &[0, 0], false, vec![0, 0]);
    test(&[], &[], true, &[0, 0], true, vec![0, 0]);
    test(&[3], &[2], false, &[0, 0], false, vec![1, 0]);
    test(&[3], &[2], true, &[0, 0], false, vec![0, 0]);
    test(&[2], &[3], false, &[0, 0], true, vec![u32::MAX, 0]);
    test(&[3], &[3], true, &[0, 0], true, vec![u32::MAX, 0]);
    test(&[2], &[3], true, &[0, 0], true, vec![u32::MAX - 1, 0]);
    test(
        &[1, 2, 3],
        &[1, 1, 1],
        false,
        &[0, 1, 2, 5],
        false,
        vec![0, 1, 2, 5],
    );
    test(
        &[1, 1, 1],
        &[1, 2, 3],
        false,
        &[5, 5, 5, 5],
        true,
        vec![0, u32::MAX, 0xfffffffd, 5],
    );
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        false,
        &[10, 10, 10, 10],
        false,
        vec![u32::MAX - 1, u32::MAX, 1, 10],
    );
    test(
        &[102, 101, 100],
        &[100, 101, 102],
        false,
        &[10, 10, 10, 10],
        true,
        vec![2, 0, u32::MAX - 1, 10],
    );
    test(
        &[0, 0, 0],
        &[1, 0, 0],
        false,
        &[10, 10, 10, 10],
        true,
        vec![u32::MAX, u32::MAX, u32::MAX, 10],
    );
    test(
        &[0, 0, 0],
        &[1, 0, 0],
        true,
        &[10, 10, 10, 10],
        true,
        vec![u32::MAX - 1, u32::MAX, u32::MAX, 10],
    );
    test(
        &[0, 0, 0],
        &[0, 0, 0],
        true,
        &[10, 10, 10, 10],
        true,
        vec![u32::MAX, u32::MAX, u32::MAX, 10],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_same_length_with_borrow_in_to_out_fail_1() {
    let mut out = vec![10, 10];
    limbs_sub_same_length_with_borrow_in_to_out(&mut out, &[6, 7, 8], &[1, 2, 3], false);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_same_length_with_borrow_in_to_out_fail_2() {
    let mut out = vec![10, 10, 10];
    limbs_sub_same_length_with_borrow_in_to_out(&mut out, &[6, 7, 8], &[1, 2], false);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_sub_same_length_with_borrow_in_in_place_left() {
    let test = |xs_before: &[Limb], ys, borrow_in, borrow, xs_after| {
        let mut xs = xs_before.to_vec();
        assert_eq!(
            limbs_sub_same_length_with_borrow_in_in_place_left(&mut xs, ys, borrow_in),
            borrow
        );
        assert_eq!(xs, xs_after);
    };
    test(&[], &[], false, false, vec![]);
    test(&[], &[], true, true, vec![]);
    test(&[3], &[2], false, false, vec![1]);
    test(&[3], &[2], true, false, vec![0]);
    test(&[2], &[3], false, true, vec![u32::MAX]);
    test(&[2], &[2], true, true, vec![u32::MAX]);
    test(&[2], &[3], true, true, vec![u32::MAX - 1]);
    test(&[1, 2, 3], &[1, 1, 1], false, false, vec![0, 1, 2]);
    test(
        &[1, 1, 1],
        &[1, 2, 3],
        false,
        true,
        vec![0, u32::MAX, 0xfffffffd],
    );
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        false,
        false,
        vec![u32::MAX - 1, u32::MAX, 1],
    );
    test(
        &[102, 101, 100],
        &[100, 101, 102],
        false,
        true,
        vec![2, 0, u32::MAX - 1],
    );
    test(
        &[0, 0, 0],
        &[1, 0, 0],
        false,
        true,
        vec![u32::MAX, u32::MAX, u32::MAX],
    );
    test(
        &[0, 0, 0],
        &[1, 0, 0],
        true,
        true,
        vec![u32::MAX - 1, u32::MAX, u32::MAX],
    );
    test(
        &[0, 0, 0],
        &[0, 0, 0],
        true,
        true,
        vec![u32::MAX, u32::MAX, u32::MAX],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_same_length_with_borrow_in_in_place_left_fail() {
    limbs_sub_same_length_with_borrow_in_in_place_left(&mut [6, 7], &[1, 2, 3], false);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_sub_same_length_in_place_with_overlap() {
    let test = |xs_before: &[Limb], right_start: usize, borrow: bool, xs_after: &[Limb]| {
        let mut xs = xs_before.to_vec();
        assert_eq!(
            limbs_sub_same_length_in_place_with_overlap(&mut xs, right_start),
            borrow
        );
        assert_eq!(xs, xs_after);

        let mut xs = xs_before.to_vec();
        assert_eq!(
            limbs_sub_same_length_in_place_with_overlap_naive(&mut xs, right_start),
            borrow
        );
        assert_eq!(xs, xs_after);
    };
    test(&[], 0, false, &[]);

    test(&[4, 3, 2, 1], 0, false, &[0, 0, 0, 0]);
    test(&[4, 3, 2, 1], 1, false, &[1, 1, 1, 1]);
    test(&[4, 3, 2, 1], 2, false, &[2, 2, 2, 1]);
    test(&[4, 3, 2, 1], 3, false, &[3, 3, 2, 1]);
    test(&[4, 3, 2, 1], 4, false, &[4, 3, 2, 1]);

    test(&[1, 2, 3, 4], 0, false, &[0, 0, 0, 0]);
    test(
        &[1, 2, 3, 4],
        1,
        true,
        &[u32::MAX, u32::MAX - 1, u32::MAX - 1, 4],
    );
    test(&[1, 2, 3, 4], 2, true, &[u32::MAX - 1, 0xfffffffd, 3, 4]);
    test(&[1, 2, 3, 4], 3, true, &[0xfffffffd, 2, 3, 4]);
    test(&[1, 2, 3, 4], 4, false, &[1, 2, 3, 4]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_same_length_in_place_with_overlap_fail() {
    limbs_sub_same_length_in_place_with_overlap(&mut [1, 2, 3], 4);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_sub_same_length_to_out_with_overlap() {
    let test = |xs_before: &[Limb], ys: &[Limb], borrow: bool, xs_after: &[Limb]| {
        let mut xs = xs_before.to_vec();
        assert_eq!(
            limbs_sub_same_length_to_out_with_overlap(&mut xs, ys),
            borrow
        );
        assert_eq!(xs, xs_after);

        let mut xs = xs_before.to_vec();
        assert_eq!(
            limbs_sub_same_length_to_out_with_overlap_naive(&mut xs, ys),
            borrow
        );
        assert_eq!(xs, xs_after);
    };
    test(&[], &[], false, &[]);
    test(&[5], &[], false, &[5]);
    test(&[5], &[3], false, &[2]);
    test(&[5, 5, 5], &[3, 3], false, &[2, 2, 5]);
    test(&[1, 2, 3, 4], &[], false, &[1, 2, 3, 4]);
    test(&[1, 2, 3, 4], &[2], false, &[2, 2, 3, 4]);
    test(&[1, 2, 3, 4], &[2, 2], false, &[1, 2, 3, 4]);
    test(&[1, 2, 3, 4], &[2, 2, 2], false, &[0, 1, 2, 4]);
    test(
        &[1, 2, 3, 4],
        &[2, 2, 2, 2],
        false,
        &[u32::MAX, u32::MAX, 0, 2],
    );
    test(&[1, 2, 3, 4], &[], false, &[1, 2, 3, 4]);
    test(&[1, 2, 3, 4], &[4], false, &[0, 2, 3, 4]);
    test(&[1, 2, 3, 4], &[4, 4], true, &[u32::MAX, u32::MAX, 3, 4]);
    test(
        &[1, 2, 3, 4],
        &[4, 4, 4],
        true,
        &[u32::MAX - 1, u32::MAX - 1, u32::MAX, 4],
    );
    test(
        &[1, 2, 3, 4],
        &[4, 4, 4, 4],
        true,
        &[0xfffffffd, 0xfffffffd, u32::MAX - 1, u32::MAX],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_same_length_to_out_with_overlap_fail() {
    limbs_sub_same_length_to_out_with_overlap(&mut [1, 2, 3], &[1, 2, 3, 4]);
}

#[test]
fn test_sub_natural() {
    let test = |s, t, out| {
        let u = Natural::from_str(s).unwrap();
        let v = Natural::from_str(t).unwrap();

        let mut n = u.clone();
        n -= v.clone();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = u.clone();
        n -= &v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone() - v.clone();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone() - &v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &u - v.clone();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &u - &v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = BigUint::from_str(s).unwrap() - BigUint::from_str(t).unwrap();
        assert_eq!(n.to_string(), out);

        let n = rug::Integer::from_str(s).unwrap() - rug::Integer::from_str(t).unwrap();
        assert_eq!(n.to_string(), out);
    };
    test("0", "0", "0");
    test("123", "0", "123");
    test("456", "123", "333");
    test("1000000000000", "123", "999999999877");
    test("12345678987654321", "314159265358979", "12031519722295342");
    test("4294967296", "1", "4294967295");
    test("4294967295", "4294967295", "0");
    test("4294967296", "4294967295", "1");
    test("4294967296", "4294967296", "0");
    test("18446744073709551616", "1", "18446744073709551615");
    test("18446744073709551615", "18446744073709551615", "0");
    test("18446744073709551616", "18446744073709551615", "1");
    test("70734740290631708", "282942734368", "70734457347897340");
}

#[test]
#[should_panic]
fn sub_assign_fail() {
    let mut x = Natural::from_str("123").unwrap();
    x -= Natural::from_str("456").unwrap();
}

#[test]
#[should_panic]
fn sub_assign_ref_fail() {
    let mut x = Natural::from_str("123").unwrap();
    x -= &Natural::from_str("456").unwrap();
}

#[test]
#[should_panic]
#[allow(unused_must_use, clippy::unnecessary_operation)]
fn sub_fail_1() {
    Natural::from(123u32) - Natural::from(456u32);
}

#[test]
#[should_panic]
#[allow(unused_must_use, clippy::unnecessary_operation)]
fn sub_fail_2() {
    Natural::from(123u32) - &Natural::from(456u32);
}

#[test]
#[should_panic]
#[allow(unused_must_use, clippy::unnecessary_operation)]
fn sub_fail_3() {
    &Natural::from(123u32) - Natural::from(456u32);
}

#[test]
#[should_panic]
#[allow(unused_must_use, clippy::unnecessary_operation)]
fn sub_fail_4() {
    &Natural::from(123u32) - &Natural::from(456u32);
}

#[test]
fn limbs_sub_limb_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen().test_properties_with_config(&config, |(xs, y)| {
        let (result, borrow) = limbs_sub_limb(&xs, y);
        if borrow {
            if xs.is_empty() {
                assert_ne!(y, 0);
                assert!(result.is_empty());
            } else {
                let mut result = result;
                result.push(Limb::MAX);
                assert_eq!(
                    Integer::from_owned_twos_complement_limbs_asc(result),
                    Integer::from(Natural::from_owned_limbs_asc(xs)) - Integer::from(y)
                );
            }
        } else {
            assert_eq!(
                Natural::from_owned_limbs_asc(result),
                Natural::from_owned_limbs_asc(xs) - Natural::from(y)
            );
        }
    });
}

#[test]
fn limbs_sub_limb_to_out_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_1().test_properties_with_config(
        &config,
        |(mut out, xs, y)| {
            let old_out = out.clone();
            let len = xs.len();
            if limbs_sub_limb_to_out(&mut out, &xs, y) {
                let n = Integer::from(Natural::from_owned_limbs_asc(xs)) - Integer::from(y);
                let mut limbs = n.into_twos_complement_limbs_asc();
                limbs.resize(len, Limb::MAX);
                assert_eq!(limbs, &out[..len]);
                assert_eq!(&out[len..], &old_out[len..]);
            } else {
                let n = Natural::from_owned_limbs_asc(xs) - Natural::from(y);
                let mut xs = n.into_limbs_asc();
                xs.resize(len, 0);
                assert_eq!(xs, &out[..len]);
                assert_eq!(&out[len..], &old_out[len..]);
            }
        },
    );
}

#[test]
fn limbs_sub_limb_in_place_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen().test_properties_with_config(&config, |(mut xs, y)| {
        let old_xs = xs.clone();
        if limbs_sub_limb_in_place(&mut xs, y) {
            let n = Integer::from(Natural::from_owned_limbs_asc(old_xs)) - Integer::from(y);
            let mut expected_xs = n.into_twos_complement_limbs_asc();
            expected_xs.resize(xs.len(), Limb::MAX);
            assert_eq!(xs, expected_xs);
        } else {
            let n = Natural::from_owned_limbs_asc(old_xs) - Natural::from(y);
            let mut expected_xs = n.into_limbs_asc();
            expected_xs.resize(xs.len(), 0);
            assert_eq!(xs, expected_xs);
        }
    });
}

#[test]
fn limbs_sub_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen_var_31().test_properties_with_config(&config, |(xs, ys)| {
        let (out, borrow) = limbs_sub(&xs, &ys);
        let len = out.len();
        let n = Natural::from_owned_limbs_asc(out);
        if borrow {
            assert_eq!(
                n,
                Natural::from_owned_limbs_asc(xs)
                    + Natural::from_owned_limbs_asc(ys)
                        .mod_power_of_2_neg(u64::exact_from(len) << Limb::LOG_WIDTH)
            );
        } else {
            assert_eq!(
                n,
                Natural::from_owned_limbs_asc(xs) - Natural::from_owned_limbs_asc(ys)
            );
        }
    });
}

fn limbs_sub_greater_to_out_helper(
    f: &mut dyn FnMut(&mut [Limb], &[Limb], &[Limb]) -> bool,
    mut out: Vec<Limb>,
    xs: Vec<Limb>,
    ys: Vec<Limb>,
) {
    let old_out = out.clone();
    let len = xs.len();
    let mut result_xs = if f(&mut out, &xs, &ys) {
        let n = Natural::from_owned_limbs_asc(xs)
            + Natural::from_owned_limbs_asc(ys)
                .mod_power_of_2_neg(u64::exact_from(len) << Limb::LOG_WIDTH);
        n.into_limbs_asc()
    } else {
        let n = Natural::from_owned_limbs_asc(xs) - Natural::from_owned_limbs_asc(ys);
        n.into_limbs_asc()
    };
    result_xs.resize(len, 0);
    assert_eq!(result_xs, &out[..len]);
    assert_eq!(&out[len..], &old_out[len..]);
}

#[test]
fn limbs_sub_same_length_to_out_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_triple_gen_var_31().test_properties_with_config(&config, |(out, xs, ys)| {
        limbs_sub_greater_to_out_helper(&mut limbs_sub_same_length_to_out, out, xs, ys);
    });
}

#[test]
fn limbs_sub_greater_to_out_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_triple_gen_var_40().test_properties_with_config(&config, |(out, xs, ys)| {
        limbs_sub_greater_to_out_helper(&mut limbs_sub_greater_to_out, out, xs, ys);
    });
}

fn limbs_sub_greater_in_place_left_helper(
    f: &mut dyn FnMut(&mut [Limb], &[Limb]) -> bool,
    mut xs: Vec<Limb>,
    ys: Vec<Limb>,
) {
    let xs_old = xs.clone();
    let len = xs.len();
    let borrow = f(&mut xs, &ys);
    let n = Natural::from_owned_limbs_asc(xs);
    if borrow {
        assert_eq!(
            n,
            Natural::from_owned_limbs_asc(xs_old)
                + Natural::from_owned_limbs_asc(ys)
                    .mod_power_of_2_neg(u64::exact_from(len) << Limb::LOG_WIDTH)
        );
    } else {
        assert_eq!(
            n,
            Natural::from_owned_limbs_asc(xs_old) - Natural::from_owned_limbs_asc(ys)
        );
    }
}

#[test]
fn limbs_sub_same_length_in_place_left_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen_var_6().test_properties_with_config(&config, |(xs, ys)| {
        limbs_sub_greater_in_place_left_helper(&mut limbs_sub_same_length_in_place_left, xs, ys);
    });
}

#[test]
fn limbs_sub_greater_in_place_left_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen_var_31().test_properties_with_config(&config, |(xs, ys)| {
        limbs_sub_greater_in_place_left_helper(&mut limbs_sub_greater_in_place_left, xs, ys);
    });
}

#[test]
fn limbs_slice_sub_in_place_right_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_24().test_properties_with_config(
        &config,
        |(xs, mut ys, len)| {
            let mut ys_old = ys.clone();
            let borrow = limbs_slice_sub_in_place_right(&xs, &mut ys, len);
            let xs_len = xs.len();
            let x = Natural::from_owned_limbs_asc(xs);
            ys_old.truncate(len);
            let y = Natural::from_owned_limbs_asc(ys_old);
            ys.truncate(xs_len);
            let n = Natural::from_owned_limbs_asc(ys);
            if borrow {
                assert_eq!(
                    n,
                    x + y.mod_power_of_2_neg(u64::exact_from(xs_len) << Limb::LOG_WIDTH)
                );
            } else {
                assert_eq!(n, x - y);
            }
        },
    );

    unsigned_vec_pair_gen_var_6().test_properties_with_config(&config, |(xs, mut ys)| {
        assert!(!limbs_slice_sub_in_place_right(&xs, &mut ys, 0));
        assert_eq!(xs, ys);
    });
}

macro_rules! limbs_vec_sub_in_place_right_helper {
    ($f:ident, $xs:ident, $ys:ident) => {
        let ys_old = $ys.clone();
        let borrow = $f(&$xs, &mut $ys);
        let n = Natural::from_owned_limbs_asc($ys);
        if borrow {
            let xs_len = u64::exact_from($xs.len());
            assert_eq!(
                n,
                Natural::from_owned_limbs_asc($xs)
                    + Natural::from_owned_limbs_asc(ys_old)
                        .mod_power_of_2_neg(xs_len << Limb::LOG_WIDTH)
            );
        } else {
            assert_eq!(
                n,
                Natural::from_owned_limbs_asc($xs) - Natural::from_owned_limbs_asc(ys_old)
            );
        }
    };
}

#[test]
fn limbs_sub_same_length_in_place_right_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen_var_6().test_properties_with_config(&config, |(xs, mut ys)| {
        limbs_vec_sub_in_place_right_helper!(limbs_sub_same_length_in_place_right, xs, ys);
    });
}

#[test]
fn limbs_vec_sub_in_place_right_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen_var_31().test_properties_with_config(&config, |(xs, mut ys)| {
        limbs_vec_sub_in_place_right_helper!(limbs_vec_sub_in_place_right, xs, ys);
    });
}

#[test]
fn limbs_sub_same_length_with_borrow_in_to_out_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    large_type_gen_var_9().test_properties_with_config(&config, |(mut out, xs, ys, borrow_in)| {
        let old_out = out.clone();
        let len = xs.len();
        let n = if limbs_sub_same_length_with_borrow_in_to_out(&mut out, &xs, &ys, borrow_in) {
            let mut n = Integer::from(Natural::from_owned_limbs_asc(xs))
                - Integer::from(Natural::from_owned_limbs_asc(ys));
            if borrow_in {
                n -= Integer::ONE;
            }
            assert!(n < 0);
            n.mod_power_of_2(u64::exact_from(len) << Limb::LOG_WIDTH)
        } else {
            let mut n = Natural::from_owned_limbs_asc(xs) - Natural::from_owned_limbs_asc(ys);
            if borrow_in {
                n -= Natural::ONE;
            }
            n
        };
        let mut limbs = n.into_limbs_asc();
        limbs.resize(len, 0);
        assert_eq!(limbs, &out[..len]);
        assert_eq!(&out[len..], &old_out[len..]);
    });
}

#[test]
fn limbs_sub_same_length_with_borrow_in_in_place_left_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_vec_bool_triple_gen_var_1().test_properties_with_config(
        &config,
        |(mut xs, ys, borrow_in)| {
            let xs_old = xs.clone();
            let len = xs.len();
            let borrow =
                limbs_sub_same_length_with_borrow_in_in_place_left(&mut xs, &ys, borrow_in);
            let n = Natural::from_owned_limbs_asc(xs);
            let mut expected_result = if borrow {
                let bit_len = u64::exact_from(len) << Limb::LOG_WIDTH;
                let mut neg_y = Natural::from_owned_limbs_asc(ys).mod_power_of_2_neg(bit_len);
                if neg_y == 0 {
                    neg_y = Natural::power_of_2(bit_len);
                }
                Natural::from_owned_limbs_asc(xs_old) + neg_y
            } else {
                Natural::from_owned_limbs_asc(xs_old) - Natural::from_owned_limbs_asc(ys)
            };
            if borrow_in {
                expected_result -= Natural::ONE;
            }
            assert_eq!(n, expected_result);
        },
    );
}

#[test]
fn limbs_sub_same_length_in_place_with_overlap_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_1().test_properties_with_config(
        &config,
        |(mut xs, right_start)| {
            let xs_old = xs.clone();
            let borrow = limbs_sub_same_length_in_place_with_overlap(&mut xs, right_start);
            let len = xs.len() - right_start;
            let x = Natural::from_limbs_asc(&xs_old[..len]);
            let y = Natural::from_limbs_asc(&xs_old[right_start..]);
            let n = Natural::from_limbs_asc(&xs[..len]);
            if borrow {
                assert_eq!(
                    n,
                    x + y.mod_power_of_2_neg(u64::exact_from(len) << Limb::LOG_WIDTH)
                );
            } else {
                assert_eq!(n, x - y);
            }
            assert_eq!(&xs[len..], &xs_old[len..]);
            let mut xs_alt = xs_old;
            assert_eq!(
                limbs_sub_same_length_in_place_with_overlap_naive(&mut xs_alt, right_start),
                borrow
            );
            assert_eq!(xs_alt, xs);
        },
    );

    unsigned_vec_gen().test_properties_with_config(&config, |mut xs| {
        let xs_old = xs.clone();
        assert!(!limbs_sub_same_length_in_place_with_overlap(&mut xs, 0));
        assert!(slice_test_zero(&xs));
        let mut xs = xs_old.clone();
        assert!(!limbs_sub_same_length_in_place_with_overlap(
            &mut xs,
            xs_old.len(),
        ));
        assert_eq!(xs, xs_old);
    });
}

#[test]
fn limbs_sub_same_length_to_out_with_overlap_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen_var_31().test_properties_with_config(&config, |(mut xs, ys)| {
        let xs_old = xs.clone();
        let borrow = limbs_sub_same_length_to_out_with_overlap(&mut xs, &ys);
        let len = ys.len();
        let x = Natural::from_limbs_asc(&xs_old[xs.len() - len..]);
        let y = Natural::from_limbs_asc(&ys);
        let n = Natural::from_limbs_asc(&xs[..len]);
        if borrow {
            assert_eq!(
                n,
                x + y.mod_power_of_2_neg(u64::exact_from(len) << Limb::LOG_WIDTH)
            );
        } else {
            assert_eq!(n, x - y);
        }
        if len <= xs.len() - len {
            assert_eq!(&xs[len..xs.len() - len], &xs_old[len..xs.len() - len]);
        }

        let mut xs_alt = xs_old;
        assert_eq!(
            limbs_sub_same_length_to_out_with_overlap_naive(&mut xs_alt, &ys),
            borrow
        );
        assert_eq!(xs_alt, xs);
    });

    unsigned_vec_gen().test_properties_with_config(&config, |mut xs| {
        let xs_old = xs.clone();
        assert!(!limbs_sub_same_length_to_out_with_overlap(&mut xs, &xs_old));
        assert!(slice_test_zero(&xs));
        let mut xs = xs_old.clone();
        assert!(!limbs_sub_same_length_to_out_with_overlap(&mut xs, &[]));
        assert_eq!(xs, xs_old);
    });
}

#[test]
fn sub_properties() {
    natural_pair_gen_var_10().test_properties(|(x, y)| {
        let mut mut_x = x.clone();
        mut_x -= y.clone();
        assert!(mut_x.is_valid());
        let diff = mut_x;

        let mut mut_x = x.clone();
        mut_x -= &y;
        assert!(mut_x.is_valid());
        let diff_alt = mut_x;
        assert_eq!(diff_alt, diff);

        let mut rug_x = rug::Integer::from(&x);
        rug_x -= rug::Integer::from(&y);
        assert_eq!(Natural::exact_from(&rug_x), diff);

        let diff_alt = x.clone() - y.clone();
        assert!(diff_alt.is_valid());
        assert_eq!(diff_alt, diff);

        let diff_alt = x.clone() - &y;
        assert_eq!(diff_alt, diff);
        assert!(diff_alt.is_valid());

        let diff_alt = &x - y.clone();
        assert_eq!(diff_alt, diff);
        assert!(diff_alt.is_valid());

        let diff_alt = &x - &y;
        assert_eq!(diff_alt, diff);
        assert!(diff_alt.is_valid());

        assert_eq!(
            Natural::from(&(BigUint::from(&x) - BigUint::from(&y))),
            diff
        );
        assert_eq!(
            Natural::exact_from(&(rug::Integer::from(&x) - rug::Integer::from(&y))),
            diff
        );

        assert!(diff <= x);
        assert_eq!(diff + y, x);
    });

    natural_gen().test_properties(|x| {
        assert_eq!(&x - Natural::ZERO, x);
        assert_eq!(&x - &x, Natural::ZERO);
    });

    unsigned_pair_gen_var_7::<Limb>().test_properties(|(y, x)| {
        assert_eq!(Natural::from(x - y), Natural::from(x) - Natural::from(y));
    });
}
