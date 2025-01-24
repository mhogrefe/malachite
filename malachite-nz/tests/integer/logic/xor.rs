// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{NegativeOne, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{
    signed_pair_gen, unsigned_vec_pair_gen_var_8, unsigned_vec_triple_gen_var_34,
    unsigned_vec_unsigned_pair_gen_var_15, unsigned_vec_unsigned_pair_gen_var_18,
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_4,
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_5,
};
use malachite_nz::integer::logic::xor::{
    limbs_neg_xor_limb, limbs_neg_xor_limb_neg, limbs_neg_xor_limb_neg_in_place,
    limbs_neg_xor_limb_neg_to_out, limbs_neg_xor_limb_to_out, limbs_pos_xor_limb_neg,
    limbs_pos_xor_limb_neg_to_out, limbs_slice_neg_xor_limb_in_place,
    limbs_slice_pos_xor_limb_neg_in_place, limbs_vec_neg_xor_limb_in_place,
    limbs_vec_pos_xor_limb_neg_in_place, limbs_xor_neg_neg, limbs_xor_neg_neg_in_place_either,
    limbs_xor_neg_neg_in_place_left, limbs_xor_neg_neg_to_out, limbs_xor_pos_neg,
    limbs_xor_pos_neg_in_place_either, limbs_xor_pos_neg_in_place_left,
    limbs_xor_pos_neg_in_place_right, limbs_xor_pos_neg_to_out,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::{Limb, SignedLimb};
use malachite_nz::test_util::generators::{
    integer_gen, integer_pair_gen, integer_triple_gen, natural_pair_gen,
};
use malachite_nz::test_util::integer::logic::xor::{integer_xor_alt_1, integer_xor_alt_2};
use rug;
use std::cmp::max;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_neg_xor_limb_and_limbs_vec_neg_xor_limb_in_place() {
    let test = |xs: &[Limb], y: Limb, out: &[Limb]| {
        assert_eq!(limbs_neg_xor_limb(xs, y), out);

        let mut xs = xs.to_vec();
        limbs_vec_neg_xor_limb_in_place(&mut xs, y);
        assert_eq!(xs, out);
    };
    test(&[6, 7], 0, &[6, 7]);
    test(&[6, 7], 2, &[8, 7]);
    test(&[100, 101, 102], 10, &[106, 101, 102]);
    test(&[123, 456], 789, &[880, 456]);
    test(&[Limb::MAX - 1, Limb::MAX, Limb::MAX], 2, &[0, 0, 0, 1]);
    test(&[0, 0, 0, 1], 2, &[Limb::MAX - 1, Limb::MAX, Limb::MAX, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_slice_neg_xor_limb_in_place() {
    let test = |xs: &[Limb], y: Limb, out: &[Limb]| {
        let mut xs = xs.to_vec();
        limbs_slice_neg_xor_limb_in_place(&mut xs, y);
        assert_eq!(xs, out);
    };
    test(&[6, 7], 0, &[6, 7]);
    test(&[6, 7], 2, &[8, 7]);
    test(&[100, 101, 102], 10, &[106, 101, 102]);
    test(&[123, 456], 789, &[880, 456]);
    test(&[Limb::MAX - 1, Limb::MAX, Limb::MAX], 2, &[0, 0, 0]);
    test(&[0, 0, 0, 1], 2, &[Limb::MAX - 1, Limb::MAX, Limb::MAX, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_neg_xor_limb_to_out() {
    let test = |out_before: &[Limb], xs: &[Limb], y: Limb, carry, out_after: &[Limb]| {
        let mut out = out_before.to_vec();
        assert_eq!(limbs_neg_xor_limb_to_out(&mut out, xs, y), carry);
        assert_eq!(out, out_after);
    };
    test(&[10, 10], &[1], Limb::MAX, true, &[0, 10]);
    test(&[10; 4], &[6, 7], 0, false, &[6, 7, 10, 10]);
    test(&[10; 4], &[6, 7], 2, false, &[8, 7, 10, 10]);
    test(&[10; 4], &[100, 101, 102], 10, false, &[106, 101, 102, 10]);
    test(&[10; 4], &[123, 456], 789, false, &[880, 456, 10, 10]);
    test(
        &[10; 4],
        &[Limb::MAX - 1, Limb::MAX, Limb::MAX],
        2,
        true,
        &[0, 0, 0, 10],
    );
    test(
        &[10; 4],
        &[0, 0, 0, 1],
        2,
        false,
        &[Limb::MAX - 1, Limb::MAX, Limb::MAX, 0],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_neg_xor_limb_to_out_fail() {
    limbs_neg_xor_limb_to_out(&mut [10], &[10, 10], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_pos_xor_limb_neg_and_limbs_vec_pos_xor_limb_neg_in_place() {
    let test = |xs: &[Limb], y: Limb, out: &[Limb]| {
        assert_eq!(limbs_pos_xor_limb_neg(xs, y), out);

        let mut xs = xs.to_vec();
        limbs_vec_pos_xor_limb_neg_in_place(&mut xs, y);
        assert_eq!(xs, out);
    };
    test(&[0, 2], 3, &[0xfffffffd, 2]);
    test(&[1, 2, 3], 4, &[0xfffffffb, 2, 3]);
    test(&[2, Limb::MAX], 2, &[0, 0, 1]);
    test(&[2, Limb::MAX, Limb::MAX], 2, &[0, 0, 0, 1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_xor_limb_neg_fail() {
    limbs_pos_xor_limb_neg(&[], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_vec_pos_xor_limb_neg_in_place_fail() {
    let mut xs = vec![];
    limbs_vec_pos_xor_limb_neg_in_place(&mut xs, 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_pos_xor_limb_neg_to_out() {
    let test = |out_before: &[Limb], xs: &[Limb], y: Limb, carry, out_after: &[Limb]| {
        let mut out = out_before.to_vec();
        assert_eq!(limbs_pos_xor_limb_neg_to_out(&mut out, xs, y), carry);
        assert_eq!(out, out_after);
    };
    test(&[0, 0], &[0, 2], 3, false, &[0xfffffffd, 2]);
    test(&[1, 2, 100], &[0, 2, 100], 3, false, &[0xfffffffd, 2, 100]);
    test(&[0, 0, 0], &[1, 2, 3], 4, false, &[0xfffffffb, 2, 3]);
    test(&[0, 0], &[2, Limb::MAX], 2, true, &[0, 0]);
    test(&[0, 0, 0], &[2, Limb::MAX, Limb::MAX], 2, true, &[0, 0, 0]);
    test(
        &[1, 2, 3, 100],
        &[2, Limb::MAX, Limb::MAX],
        2,
        true,
        &[0, 0, 0, 100],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_xor_limb_neg_to_out_fail_1() {
    limbs_pos_xor_limb_neg_to_out(&mut [1, 2, 3], &[1, 2, 3, 4], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_xor_limb_neg_to_out_fail_2() {
    limbs_pos_xor_limb_neg_to_out(&mut [1, 2, 3], &[], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_slice_pos_xor_limb_neg_in_place() {
    let test = |xs_before: &[Limb], y: Limb, carry, xs_after: &[Limb]| {
        let mut xs = xs_before.to_vec();
        assert_eq!(limbs_slice_pos_xor_limb_neg_in_place(&mut xs, y), carry);
        assert_eq!(xs, xs_after);
    };
    test(&[0, 2], 3, false, &[0xfffffffd, 2]);
    test(&[1, 2, 3], 4, false, &[0xfffffffb, 2, 3]);
    test(&[2, Limb::MAX], 2, true, &[0, 0]);
    test(&[2, Limb::MAX, Limb::MAX], 2, true, &[0, 0, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_slice_pos_xor_limb_neg_in_place_fail() {
    limbs_slice_pos_xor_limb_neg_in_place(&mut [], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_neg_xor_limb_neg_and_limbs_neg_xor_limb_neg_in_place() {
    let test = |xs: &[Limb], y: Limb, out: &[Limb]| {
        assert_eq!(limbs_neg_xor_limb_neg(xs, y), out);

        let mut xs = xs.to_vec();
        limbs_neg_xor_limb_neg_in_place(&mut xs, y);
        assert_eq!(xs, out);
    };
    test(&[0, 2], 3, &[3, 1]);
    test(&[6, 7], 2, &[0xfffffff8, 7]);
    test(&[1, 2, 3], 4, &[0xfffffffb, 2, 3]);
    test(&[100, 101, 102], 10, &[4294967190, 101, 102]);
    test(&[123, 456], 789, &[4294966416, 456]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_neg_xor_limb_neg_fail() {
    limbs_neg_xor_limb_neg(&[], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_neg_xor_limb_neg_in_place_fail() {
    limbs_neg_xor_limb_neg_in_place(&mut [], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_neg_xor_limb_neg_to_out() {
    let test = |out_before: &[Limb], xs: &[Limb], y: Limb, out_after: &[Limb]| {
        let mut out = out_before.to_vec();
        limbs_neg_xor_limb_neg_to_out(&mut out, xs, y);
        assert_eq!(out, out_after);
    };
    test(&[10; 4], &[0, 2], 3, &[3, 1, 10, 10]);
    test(&[10; 4], &[6, 7], 2, &[0xfffffff8, 7, 10, 10]);
    test(&[10; 4], &[1, 2, 3], 4, &[0xfffffffb, 2, 3, 10]);
    test(&[10; 4], &[100, 101, 102], 10, &[4294967190, 101, 102, 10]);
    test(&[10; 4], &[123, 456], 789, &[4294966416, 456, 10, 10]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_neg_xor_limb_neg_to_out_fail_1() {
    limbs_neg_xor_limb_neg_to_out(&mut [], &[], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_neg_xor_limb_neg_to_out_fail_2() {
    limbs_neg_xor_limb_neg_to_out(&mut [10], &[10, 10], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_xor_pos_neg_limbs_xor_pos_neg_in_place_left_and_limbs_xor_pos_neg_in_place_right() {
    let test = |xs_before, ys_before, out| {
        assert_eq!(limbs_xor_pos_neg(xs_before, ys_before), out);

        let mut xs = xs_before.to_vec();
        limbs_xor_pos_neg_in_place_left(&mut xs, ys_before);
        assert_eq!(xs, out);

        let mut ys = ys_before.to_vec();
        limbs_xor_pos_neg_in_place_right(xs_before, &mut ys);
        assert_eq!(ys, out);
    };
    test(&[2], &[3], vec![1]);
    test(&[1, 1, 1], &[1, 2, 3], vec![2, 3, 2]);
    test(&[6, 7], &[1, 2, 3], vec![7, 5, 3]);
    test(&[1, 2, 3], &[6, 7], vec![5, 5, 3]);
    test(&[100, 101, 102], &[102, 101, 100], vec![2, 0, 2]);
    test(&[0, 0, 1], &[3], vec![3, 0, 1]);
    test(&[3], &[0, 0, 1], vec![0xfffffffd, Limb::MAX, 0]);
    test(&[0, 3, 3], &[0, 0, 3], vec![0, 0xfffffffd, 1]);
    test(&[0, 0, 3], &[0, 3, 3], vec![0, 3, 0]);
    test(&[0, 3], &[0, 0, 3], vec![0, 0xfffffffd, 2]);
    test(&[0, 0, 3], &[0, 3], vec![0, 3, 3]);
    test(&[0, 1], &[0, Limb::MAX, Limb::MAX], vec![0, 0, 0, 1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_xor_pos_neg_fail_1() {
    limbs_xor_pos_neg(&[0, 0, 0], &[3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_xor_pos_neg_fail_2() {
    limbs_xor_pos_neg(&[3], &[0, 0, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_xor_pos_neg_in_place_left_fail_1() {
    limbs_xor_pos_neg_in_place_left(&mut vec![0, 0, 0], &[3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_xor_pos_neg_in_place_left_fail_2() {
    limbs_xor_pos_neg_in_place_left(&mut vec![3], &[0, 0, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_xor_pos_neg_in_place_right_fail_1() {
    limbs_xor_pos_neg_in_place_right(&[0, 0, 0], &mut vec![3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_xor_pos_neg_in_place_right_fail_2() {
    limbs_xor_pos_neg_in_place_right(&[3], &mut vec![0, 0, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_xor_pos_neg_to_out() {
    let test = |xs, ys, out_before: &[Limb], out_after, carry| {
        let mut out = out_before.to_vec();
        assert_eq!(limbs_xor_pos_neg_to_out(&mut out, xs, ys), carry);
        assert_eq!(out, out_after);
    };
    test(&[2], &[3], &[10; 4], vec![1, 10, 10, 10], false);
    test(&[1, 1, 1], &[1, 2, 3], &[10; 4], vec![2, 3, 2, 10], false);
    test(&[6, 7], &[1, 2, 3], &[10; 4], vec![7, 5, 3, 10], false);
    test(&[1, 2, 3], &[6, 7], &[10; 4], vec![5, 5, 3, 10], false);
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        &[10; 4],
        vec![2, 0, 2, 10],
        false,
    );
    test(&[0, 0, 1], &[3], &[10; 4], vec![3, 0, 1, 10], false);
    test(
        &[3],
        &[0, 0, 1],
        &[10; 4],
        vec![0xfffffffd, Limb::MAX, 0, 10],
        false,
    );
    test(
        &[0, 3, 3],
        &[0, 0, 3],
        &[10; 4],
        vec![0, 0xfffffffd, 1, 10],
        false,
    );
    test(&[0, 0, 3], &[0, 3, 3], &[10; 4], vec![0, 3, 0, 10], false);
    test(
        &[0, 3],
        &[0, 0, 3],
        &[10; 4],
        vec![0, 0xfffffffd, 2, 10],
        false,
    );
    test(&[0, 0, 3], &[0, 3], &[10; 4], vec![0, 3, 3, 10], false);
    test(
        &[0, 1],
        &[0, Limb::MAX, Limb::MAX],
        &[10; 4],
        vec![0, 0, 0, 10],
        true,
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_xor_pos_neg_to_out_fail_1() {
    let mut out = vec![10; 4];
    limbs_xor_pos_neg_to_out(&mut out, &[0, 0, 0], &[3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_xor_pos_neg_to_out_fail_2() {
    let mut out = vec![10; 4];
    limbs_xor_pos_neg_to_out(&mut out, &[3], &[0, 0, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_xor_pos_neg_to_out_fail_3() {
    let mut out = vec![10];
    limbs_xor_pos_neg_to_out(&mut out, &[6, 7], &[1, 2]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_xor_pos_neg_in_place_either() {
    let test = |xs_before: &[Limb], ys_before: &[Limb], b, xs_after, ys_after| {
        let mut xs = xs_before.to_vec();
        let mut ys = ys_before.to_vec();
        assert_eq!(limbs_xor_pos_neg_in_place_either(&mut xs, &mut ys), b);
        assert_eq!(xs, xs_after);
        assert_eq!(ys, ys_after);
    };
    test(&[2], &[3], false, vec![1], vec![3]);
    test(&[1, 1, 1], &[1, 2, 3], false, vec![2, 3, 2], vec![1, 2, 3]);
    test(&[1, 2, 3], &[6, 7], false, vec![5, 5, 3], vec![6, 7]);
    test(&[6, 7], &[1, 2, 3], true, vec![6, 7], vec![7, 5, 3]);
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        false,
        vec![2, 0, 2],
        vec![102, 101, 100],
    );
    test(&[0, 0, 1], &[3], false, vec![3, 0, 1], vec![3]);
    test(
        &[3],
        &[0, 0, 1],
        true,
        vec![3],
        vec![0xfffffffd, Limb::MAX, 0],
    );
    test(
        &[0, 3, 3],
        &[0, 0, 3],
        false,
        vec![0, 0xfffffffd, 1],
        vec![0, 0, 3],
    );
    test(&[0, 0, 3], &[0, 3, 3], false, vec![0, 3, 0], vec![0, 3, 3]);
    test(
        &[0, 3],
        &[0, 0, 3],
        true,
        vec![0, 3],
        vec![0, 0xfffffffd, 2],
    );
    test(&[0, 0, 3], &[0, 3], false, vec![0, 3, 3], vec![0, 3]);
    test(
        &[1, 0, 0],
        &[Limb::MAX, Limb::MAX, Limb::MAX, Limb::MAX],
        true,
        vec![1, 0, 0],
        vec![0, 0, 0, 0, 1],
    );
    test(
        &[0, 1],
        &[0, Limb::MAX, Limb::MAX],
        true,
        vec![0, 1],
        vec![0, 0, 0, 1],
    );
    test(
        &[Limb::MAX, Limb::MAX, Limb::MAX, Limb::MAX],
        &[1, 0, 0],
        false,
        vec![0, 0, 0, 0, 1],
        vec![1, 0, 0],
    );
    test(
        &[0, Limb::MAX, Limb::MAX],
        &[0, 1],
        false,
        vec![0, 0, 0, 1],
        vec![0, 1],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_xor_pos_neg_in_place_either_fail_1() {
    limbs_xor_pos_neg_in_place_either(&mut vec![0, 0, 0], &mut vec![3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_xor_pos_neg_in_place_either_fail_2() {
    limbs_xor_pos_neg_in_place_either(&mut vec![3], &mut vec![0, 0, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_xor_neg_neg_and_limbs_xor_neg_neg_in_place_left() {
    let test = |xs_before, ys, out| {
        assert_eq!(limbs_xor_neg_neg(xs_before, ys), out);

        let mut xs = xs_before.to_vec();
        limbs_xor_neg_neg_in_place_left(&mut xs, ys);
        assert_eq!(xs, out);
    };
    test(&[2], &[3], vec![3]);
    test(&[1, 1, 1], &[1, 2, 3], vec![0, 3, 2]);
    test(&[6, 7], &[1, 2, 3], vec![5, 5, 3]);
    test(&[1, 2, 3], &[6, 7], vec![5, 5, 3]);
    test(&[100, 101, 102], &[102, 101, 100], vec![6, 0, 2]);
    test(&[0, 0, 1], &[3], vec![0xfffffffd, Limb::MAX, 0]);
    test(&[3], &[0, 0, 1], vec![0xfffffffd, Limb::MAX, 0]);
    test(&[0, 3, 3], &[0, 0, 3], vec![0, 0xfffffffd, 1]);
    test(&[0, 0, 3], &[0, 3, 3], vec![0, 0xfffffffd, 1]);
    test(&[0, 3], &[0, 0, 3], vec![0, 0xfffffffd, 2]);
    test(&[0, 0, 3], &[0, 3], vec![0, 0xfffffffd, 2]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_xor_neg_neg_in_place_left_fail_1() {
    limbs_xor_neg_neg_in_place_left(&mut vec![0, 0, 0], &[3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_xor_neg_neg_in_place_left_fail_2() {
    limbs_xor_neg_neg_in_place_left(&mut vec![3], &[0, 0, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_xor_neg_neg_to_out() {
    let test = |xs, ys, out_before: &[Limb], out_after| {
        let mut out = out_before.to_vec();
        limbs_xor_neg_neg_to_out(&mut out, xs, ys);
        assert_eq!(out, out_after);
    };
    test(&[2], &[3], &[10; 4], vec![3, 10, 10, 10]);
    test(&[1, 1, 1], &[1, 2, 3], &[10; 4], vec![0, 3, 2, 10]);
    test(&[6, 7], &[1, 2, 3], &[10; 4], vec![5, 5, 3, 10]);
    test(&[1, 2, 3], &[6, 7], &[10; 4], vec![5, 5, 3, 10]);
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        &[10; 4],
        vec![6, 0, 2, 10],
    );
    test(
        &[0, 0, 1],
        &[3],
        &[10; 4],
        vec![0xfffffffd, Limb::MAX, 0, 10],
    );
    test(
        &[3],
        &[0, 0, 1],
        &[10; 4],
        vec![0xfffffffd, Limb::MAX, 0, 10],
    );
    test(&[0, 3, 3], &[0, 0, 3], &[10; 4], vec![0, 0xfffffffd, 1, 10]);
    test(&[0, 0, 3], &[0, 3, 3], &[10; 4], vec![0, 0xfffffffd, 1, 10]);
    test(&[0, 3], &[0, 0, 3], &[10; 4], vec![0, 0xfffffffd, 2, 10]);
    test(&[0, 0, 3], &[0, 3], &[10; 4], vec![0, 0xfffffffd, 2, 10]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_xor_neg_neg_to_out_fail_1() {
    let mut out = vec![10; 4];
    limbs_xor_neg_neg_to_out(&mut out, &[0, 0, 0], &[3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_xor_neg_neg_to_out_fail_2() {
    let mut out = vec![10; 4];
    limbs_xor_neg_neg_to_out(&mut out, &[3], &[0, 0, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_xor_neg_neg_to_out_fail_3() {
    let mut out = vec![10];
    limbs_xor_neg_neg_to_out(&mut out, &[6, 7], &[1, 2]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_xor_neg_neg_in_place_either() {
    let test = |xs_before: &[Limb], ys_before: &[Limb], b, xs_after, ys_after| {
        let mut xs = xs_before.to_vec();
        let mut ys = ys_before.to_vec();
        assert_eq!(limbs_xor_neg_neg_in_place_either(&mut xs, &mut ys), b);
        assert_eq!(xs, xs_after);
        assert_eq!(ys, ys_after);
    };
    test(&[2], &[3], false, vec![3], vec![3]);
    test(&[1, 1, 1], &[1, 2, 3], false, vec![0, 3, 2], vec![1, 2, 3]);
    test(&[1, 2, 3], &[6, 7], false, vec![5, 5, 3], vec![6, 7]);
    test(&[6, 7], &[1, 2, 3], true, vec![6, 7], vec![5, 5, 3]);
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        false,
        vec![6, 0, 2],
        vec![102, 101, 100],
    );
    test(
        &[0, 0, 1],
        &[3],
        false,
        vec![0xfffffffd, Limb::MAX, 0],
        vec![3],
    );
    test(
        &[3],
        &[0, 0, 1],
        true,
        vec![3],
        vec![0xfffffffd, Limb::MAX, 0],
    );
    test(
        &[0, 3, 3],
        &[0, 0, 3],
        false,
        vec![0, 0xfffffffd, 1],
        vec![0, 0, 3],
    );
    test(
        &[0, 0, 3],
        &[0, 3, 3],
        false,
        vec![0, 0xfffffffd, 1],
        vec![0, 3, 3],
    );
    test(
        &[0, 3],
        &[0, 0, 3],
        true,
        vec![0, 3],
        vec![0, 0xfffffffd, 2],
    );
    test(
        &[0, 0, 3],
        &[0, 3],
        false,
        vec![0, 0xfffffffd, 2],
        vec![0, 3],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_xor_neg_neg_in_place_either_fail_1() {
    limbs_xor_neg_neg_in_place_either(&mut [0, 0, 0], &mut [3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_xor_neg_neg_in_place_either_fail_2() {
    limbs_xor_neg_neg_in_place_either(&mut [3], &mut [0, 0, 0]);
}

#[test]
fn test_xor() {
    let test = |s, t, out| {
        let u = Integer::from_str(s).unwrap();
        let v = Integer::from_str(t).unwrap();

        let mut n = u.clone();
        n ^= v.clone();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = u.clone();
        n ^= &v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone() ^ v.clone();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &u ^ v.clone();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone() ^ &v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &u ^ &v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        assert_eq!(integer_xor_alt_1(&u, &v).to_string(), out);
        assert_eq!(integer_xor_alt_2(&u, &v).to_string(), out);

        let n = rug::Integer::from_str(s).unwrap() ^ rug::Integer::from_str(t).unwrap();
        assert_eq!(n.to_string(), out);
    };
    test("0", "0", "0");
    test("0", "123", "123");
    test("123", "0", "123");
    test("123", "456", "435");
    test("1000000000000", "123", "1000000000123");
    test("123", "1000000000000", "1000000000123");
    test("1000000000000", "999999999999", "8191");
    test("12345678987654321", "314159265358979", "12035174921130034");
    test("0", "-123", "-123");
    test("123", "-456", "-445");
    test("1000000000000", "-123", "-1000000000123");
    test("123", "-1000000000000", "-999999999877");
    test("1000000000000", "-999999999999", "-8191");
    test(
        "12345678987654321",
        "-314159265358979",
        "-12035174921130036",
    );
    test("-123", "0", "-123");
    test("-123", "456", "-435");
    test("-1000000000000", "123", "-999999999877");
    test("-123", "1000000000000", "-1000000000123");
    test("-1000000000000", "999999999999", "-1");
    test(
        "-12345678987654321",
        "314159265358979",
        "-12035174921130036",
    );
    test("-123", "-456", "445");
    test("-1000000000000", "-123", "999999999877");
    test("-123", "-1000000000000", "999999999877");
    test("-1000000000000", "-999999999999", "1");
    test(
        "-12345678987654321",
        "-314159265358979",
        "12035174921130034",
    );

    test(
        "-58833344151816",
        "-163347918670491773353",
        "163347906017862822063",
    );
    test(
        "-163347918670491773353",
        "-58833344151816",
        "163347906017862822063",
    );
    test(
        "4722366342132156858368",
        "-35201550909443",
        "-4722366377333707767811",
    );
    test(
        "-35201550909443",
        "4722366342132156858368",
        "-4722366377333707767811",
    );
    test(
        "-26298808336",
        "170141183460469156173823577801560686592",
        "-170141183460469156173823577827859494928",
    );
    test(
        "170141183460469156173823577801560686592",
        "-26298808336",
        "-170141183460469156173823577827859494928",
    );
    test(
        "-19191697422411034898997120",
        "-748288838313422294120286634350663119087542623797248",
        "748288838313422294120286615158965696676507724800128",
    );
    test(
        "-748288838313422294120286634350663119087542623797248",
        "-19191697422411034898997120",
        "748288838313422294120286615158965696676507724800128",
    );
    test(
        "4294967296",
        "-79228162514264337589248983040",
        "-79228162514264337593543950336",
    );
    test(
        "-79228162514264337589248983040",
        "4294967296",
        "-79228162514264337593543950336",
    );
}

#[test]
fn limbs_neg_or_limb_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_18().test_properties_with_config(&config, |(xs, y)| {
        assert_eq!(
            -Natural::from_owned_limbs_asc(limbs_neg_xor_limb(&xs, y)),
            -Natural::from_owned_limbs_asc(xs) ^ Integer::from(y)
        );
    });
}

#[test]
fn limbs_neg_xor_limb_to_out_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_5().test_properties_with_config(
        &config,
        |(mut out, xs, y)| {
            let old_out = out.clone();
            let len = xs.len();
            if limbs_neg_xor_limb_to_out(&mut out, &xs, y) {
                let mut result =
                    Natural::exact_from(-(-Natural::from_owned_limbs_asc(xs) ^ Integer::from(y)))
                        .to_limbs_asc();
                result.resize(len, 0);
                assert_eq!(result, &out[..len]);
            } else {
                assert_eq!(
                    -Natural::from_limbs_asc(&out[..len]),
                    -Natural::from_owned_limbs_asc(xs) ^ Integer::from(y),
                );
            }
            assert_eq!(&out[len..], &old_out[len..]);
        },
    );
}

#[test]
fn limbs_slice_neg_xor_limb_in_place_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_18().test_properties_with_config(&config, |(mut xs, y)| {
        let old_xs = xs.clone();
        if limbs_slice_neg_xor_limb_in_place(&mut xs, y) {
            let mut result = Natural::exact_from(
                -(Integer::from(Natural::from_owned_limbs_asc(old_xs)) ^ Integer::from(y)),
            )
            .to_limbs_asc();
            result.resize(xs.len(), 0);
            assert_eq!(result, xs);
        } else {
            assert_eq!(
                -Natural::from_owned_limbs_asc(xs),
                -Natural::from_owned_limbs_asc(old_xs) ^ Integer::from(y)
            );
        }
    });
}

#[test]
fn limbs_vec_neg_xor_limb_in_place_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_18().test_properties_with_config(&config, |(mut xs, y)| {
        let old_xs = xs.clone();
        limbs_vec_neg_xor_limb_in_place(&mut xs, y);
        assert_eq!(
            -Natural::from_owned_limbs_asc(xs),
            -Natural::from_owned_limbs_asc(old_xs) ^ Integer::from(y)
        );
    });
}

#[test]
fn limbs_pos_xor_limb_neg_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_15().test_properties_with_config(&config, |(xs, y)| {
        let out = limbs_pos_xor_limb_neg(&xs, y);
        let n = Integer::from(Natural::from_owned_limbs_asc(xs))
            ^ Integer::from_owned_twos_complement_limbs_asc(vec![y, Limb::MAX]);
        assert_eq!(Natural::from_owned_limbs_asc(out), Natural::exact_from(-n));
    });
}

#[test]
fn limbs_pos_xor_limb_neg_to_out_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_4().test_properties_with_config(
        &config,
        |(mut out, xs, y)| {
            let old_out = out.clone();
            limbs_pos_xor_limb_neg_to_out(&mut out, &xs, y);
            let len = xs.len();
            let n = Integer::from(Natural::from_owned_limbs_asc(xs))
                ^ Integer::from_owned_twos_complement_limbs_asc(vec![y, Limb::MAX]);
            let mut result = Natural::exact_from(-n).into_limbs_asc();
            result.resize(len, 0);
            assert_eq!(result, &out[..len]);
            assert_eq!(&out[len..], &old_out[len..]);
        },
    );
}

#[test]
fn limbs_slice_pos_xor_limb_neg_in_place_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_15().test_properties_with_config(&config, |(mut xs, y)| {
        let n = Integer::from(Natural::from_limbs_asc(&xs))
            ^ Integer::from_owned_twos_complement_limbs_asc(vec![y, Limb::MAX]);
        let carry = limbs_slice_pos_xor_limb_neg_in_place(&mut xs, y);
        if carry {
            let result = Natural::exact_from(-n).to_limbs_asc();
            assert_eq!(xs, &result[..xs.len()]);
        } else {
            assert_eq!(Natural::from_owned_limbs_asc(xs), Natural::exact_from(-n));
        }
    });
}

#[test]
fn limbs_vec_pos_xor_limb_neg_in_place_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_15().test_properties_with_config(&config, |(mut xs, y)| {
        let n = Integer::from(Natural::from_limbs_asc(&xs))
            ^ Integer::from_owned_twos_complement_limbs_asc(vec![y, Limb::MAX]);
        limbs_vec_pos_xor_limb_neg_in_place(&mut xs, y);
        assert_eq!(Natural::from_owned_limbs_asc(xs), Natural::exact_from(-n));
    });
}

#[test]
fn limbs_neg_xor_limb_neg_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_18().test_properties_with_config(&config, |(xs, y)| {
        let out = limbs_neg_xor_limb_neg(&xs, y);
        let n = -Natural::from_owned_limbs_asc(xs)
            ^ Integer::from_owned_twos_complement_limbs_asc(vec![y, Limb::MAX]);
        assert_eq!(Natural::from_owned_limbs_asc(out), Natural::exact_from(n));
    });
}

#[test]
fn limbs_neg_xor_limb_neg_to_out_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_5().test_properties_with_config(
        &config,
        |(mut out, xs, y)| {
            let old_out = out.clone();
            limbs_neg_xor_limb_neg_to_out(&mut out, &xs, y);
            let len = xs.len();
            let n = -Natural::from_owned_limbs_asc(xs)
                ^ Integer::from_owned_twos_complement_limbs_asc(vec![y, Limb::MAX]);
            let mut limbs = Natural::exact_from(n).into_limbs_asc();
            limbs.resize(len, 0);
            assert_eq!(limbs, &out[..len]);
            assert_eq!(&out[len..], &old_out[len..]);
        },
    );
}

#[test]
fn limbs_neg_xor_limb_neg_in_place_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_18().test_properties_with_config(&config, |(mut xs, y)| {
        let n = -Natural::from_limbs_asc(&xs)
            ^ Integer::from_owned_twos_complement_limbs_asc(vec![y, Limb::MAX]);
        limbs_neg_xor_limb_neg_in_place(&mut xs, y);
        let mut expected_limbs = Natural::exact_from(n).into_limbs_asc();
        expected_limbs.resize(xs.len(), 0);
        assert_eq!(xs, expected_limbs);
    });
}

#[test]
fn limbs_xor_pos_neg_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen_var_8().test_properties_with_config(&config, |(xs, ys)| {
        assert_eq!(
            -Natural::from_owned_limbs_asc(limbs_xor_pos_neg(&xs, &ys)),
            Integer::from(Natural::from_owned_limbs_asc(xs)) ^ -Natural::from_owned_limbs_asc(ys)
        );
    });
}

#[test]
fn limbs_xor_pos_neg_to_out_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_triple_gen_var_34().test_properties_with_config(&config, |(mut out, xs, ys)| {
        let out_old = out.clone();
        let carry = limbs_xor_pos_neg_to_out(&mut out, &xs, &ys);
        let len = max(xs.len(), ys.len());
        let mut result = out[..len].to_vec();
        if carry {
            result.push(1);
        }
        assert_eq!(
            -Natural::from_owned_limbs_asc(result),
            Integer::from(Natural::from_owned_limbs_asc(xs)) ^ -Natural::from_owned_limbs_asc(ys)
        );
        assert_eq!(&out[len..], &out_old[len..]);
    });
}

#[test]
fn limbs_xor_pos_neg_in_place_left_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen_var_8().test_properties_with_config(&config, |(mut xs, ys)| {
        let xs_old = xs.clone();
        limbs_xor_pos_neg_in_place_left(&mut xs, &ys);
        assert_eq!(
            -Natural::from_owned_limbs_asc(xs),
            Integer::from(Natural::from_owned_limbs_asc(xs_old))
                ^ -Natural::from_owned_limbs_asc(ys)
        );
    });
}

#[test]
fn limbs_xor_pos_neg_in_place_right_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen_var_8().test_properties_with_config(&config, |(xs, mut ys)| {
        let ys_old = ys.clone();
        limbs_xor_pos_neg_in_place_right(&xs, &mut ys);
        assert_eq!(
            -Natural::from_owned_limbs_asc(ys),
            Integer::from(Natural::from_owned_limbs_asc(xs))
                ^ -Natural::from_owned_limbs_asc(ys_old)
        );
    });
}

#[test]
fn limbs_xor_pos_neg_in_place_either_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen_var_8().test_properties_with_config(&config, |(mut xs, mut ys)| {
        let xs_old = xs.clone();
        let ys_old = ys.clone();
        let right = limbs_xor_pos_neg_in_place_either(&mut xs, &mut ys);
        let expected =
            Integer::from(Natural::from_limbs_asc(&xs_old)) ^ -Natural::from_limbs_asc(&ys_old);
        if right {
            assert_eq!(xs, xs_old);
            assert_eq!(-Natural::from_owned_limbs_asc(ys), expected);
        } else {
            assert_eq!(-Natural::from_owned_limbs_asc(xs), expected);
            assert_eq!(ys, ys_old);
        }
    });
}

#[test]
fn limbs_xor_neg_neg_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen_var_8().test_properties_with_config(&config, |(xs, ys)| {
        assert_eq!(
            Natural::from_owned_limbs_asc(limbs_xor_neg_neg(&xs, &ys)),
            -Natural::from_owned_limbs_asc(xs) ^ -Natural::from_owned_limbs_asc(ys)
        );
    });
}

#[test]
fn limbs_xor_neg_neg_to_out_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_triple_gen_var_34().test_properties_with_config(&config, |(mut out, xs, ys)| {
        let out_old = out.clone();
        limbs_xor_neg_neg_to_out(&mut out, &xs, &ys);
        let len = max(xs.len(), ys.len());
        let result = Natural::exact_from(
            -Natural::from_owned_limbs_asc(xs) ^ -Natural::from_owned_limbs_asc(ys),
        );
        let mut expected = result.to_limbs_asc();
        expected.resize(len, 0);
        assert_eq!(&out[..len], expected.as_slice());
        assert_eq!(&out[len..], &out_old[len..]);
    });
}

#[test]
fn limbs_xor_neg_neg_in_place_left_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen_var_8().test_properties_with_config(&config, |(mut xs, ys)| {
        let xs_old = xs.clone();
        limbs_xor_neg_neg_in_place_left(&mut xs, &ys);
        assert_eq!(
            Natural::from_owned_limbs_asc(xs),
            -Natural::from_owned_limbs_asc(xs_old) ^ -Natural::from_owned_limbs_asc(ys)
        );
    });
}

#[test]
fn limbs_xor_neg_neg_in_place_either_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen_var_8().test_properties_with_config(&config, |(mut xs, mut ys)| {
        let xs_old = xs.clone();
        let ys_old = ys.clone();
        let right = limbs_xor_neg_neg_in_place_either(&mut xs, &mut ys);
        let expected = -Natural::from_limbs_asc(&xs_old) ^ -Natural::from_limbs_asc(&ys_old);
        if right {
            assert_eq!(xs, xs_old);
            assert_eq!(Natural::from_owned_limbs_asc(ys), expected);
        } else {
            assert_eq!(Natural::from_owned_limbs_asc(xs), expected);
            assert_eq!(ys, ys_old);
        }
    });
}

#[allow(clippy::eq_op)]
#[test]
fn xor_properties() {
    integer_pair_gen().test_properties(|(x, y)| {
        let result_val_val = x.clone() ^ y.clone();
        let result_val_ref = x.clone() ^ &y;
        let result_ref_val = &x ^ y.clone();
        let result = &x ^ &y;
        assert!(result_val_val.is_valid());
        assert!(result_val_ref.is_valid());
        assert!(result_ref_val.is_valid());
        assert!(result.is_valid());
        assert_eq!(result_val_val, result);
        assert_eq!(result_val_ref, result);
        assert_eq!(result_ref_val, result);

        let mut mut_x = x.clone();
        mut_x ^= y.clone();
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, result);
        let mut mut_x = x.clone();
        mut_x ^= &y;
        assert_eq!(mut_x, result);
        assert!(mut_x.is_valid());

        let mut mut_x = rug::Integer::from(&x);
        mut_x ^= rug::Integer::from(&y);
        assert_eq!(Integer::from(&mut_x), result);

        assert_eq!(
            Integer::from(&(rug::Integer::from(&x) ^ rug::Integer::from(&y))),
            result
        );

        assert_eq!(integer_xor_alt_1(&x, &y), result);
        assert_eq!(integer_xor_alt_2(&x, &y), result);

        assert_eq!(&y ^ &x, result);
        assert_eq!(&result ^ &x, y);
        assert_eq!(&result ^ &y, x);
        assert_eq!(!&x ^ !&y, result);
        assert_eq!(!(&x ^ !&y), result);
        assert_eq!(!(!x ^ y), result);
    });

    integer_gen().test_properties(|ref x| {
        assert_eq!(x ^ Integer::ZERO, *x);
        assert_eq!(Integer::ZERO ^ x, *x);
        assert_eq!(x ^ Integer::NEGATIVE_ONE, !x);
        assert_eq!(Integer::NEGATIVE_ONE ^ x, !x);
        assert_eq!(x ^ x, 0);
        assert_eq!(x ^ !x, -1);
        assert_eq!(!x ^ x, -1);
    });

    integer_triple_gen().test_properties(|(x, y, z)| {
        assert_eq!((&x ^ &y) ^ &z, x ^ (y ^ z));
    });

    signed_pair_gen::<SignedLimb>().test_properties(|(i, j)| {
        assert_eq!(Integer::from(i) ^ Integer::from(j), i ^ j);
    });

    natural_pair_gen().test_properties(|(x, y)| {
        assert_eq!(Integer::from(&x) ^ Integer::from(&y), x ^ y);
    });
}
