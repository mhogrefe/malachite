// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{
    signed_pair_gen, unsigned_vec_pair_gen_var_8, unsigned_vec_pair_gen_var_9,
    unsigned_vec_triple_gen_var_33, unsigned_vec_triple_gen_var_34,
    unsigned_vec_unsigned_pair_gen_var_15, unsigned_vec_unsigned_pair_gen_var_18,
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_4,
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_5,
};
use malachite_nz::integer::logic::and::{
    limbs_and_neg_neg, limbs_and_neg_neg_to_out, limbs_and_pos_neg,
    limbs_and_pos_neg_in_place_left, limbs_and_pos_neg_to_out, limbs_neg_and_limb_neg,
    limbs_neg_and_limb_neg_to_out, limbs_pos_and_limb_neg, limbs_pos_and_limb_neg_in_place,
    limbs_pos_and_limb_neg_to_out, limbs_slice_and_neg_neg_in_place_either,
    limbs_slice_and_neg_neg_in_place_left, limbs_slice_and_pos_neg_in_place_right,
    limbs_slice_neg_and_limb_neg_in_place, limbs_vec_and_neg_neg_in_place_either,
    limbs_vec_and_neg_neg_in_place_left, limbs_vec_and_pos_neg_in_place_right,
    limbs_vec_neg_and_limb_neg_in_place,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::{Limb, SignedLimb};
use malachite_nz::test_util::generators::{
    integer_gen, integer_pair_gen, integer_triple_gen, natural_pair_gen,
};
use malachite_nz::test_util::integer::logic::and::{integer_and_alt_1, integer_and_alt_2};
use rug;
use std::cmp::{max, min};
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_pos_and_limb_neg_and_limbs_pos_and_limb_neg_in_place() {
    let test = |xs: &[Limb], y: Limb, out: &[Limb]| {
        assert_eq!(limbs_pos_and_limb_neg(xs, y), out);

        let mut xs = xs.to_vec();
        limbs_pos_and_limb_neg_in_place(&mut xs, y);
        assert_eq!(xs, out);
    };
    test(&[6, 7], 2, &[2, 7]);
    test(&[100, 101, 102], 10, &[0, 101, 102]);
    test(&[123, 456], 789, &[17, 456]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_and_limb_neg_fail() {
    limbs_pos_and_limb_neg(&[], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_pos_and_limb_neg_to_out() {
    let test = |out_before: &[Limb], xs: &[Limb], y: Limb, out_after: &[Limb]| {
        let mut out = out_before.to_vec();
        limbs_pos_and_limb_neg_to_out(&mut out, xs, y);
        assert_eq!(out, out_after);
    };
    test(&[10, 10, 10, 10], &[6, 7], 2, &[2, 7, 10, 10]);
    test(&[10, 10, 10, 10], &[100, 101, 102], 10, &[0, 101, 102, 10]);
    test(&[10, 10, 10, 10], &[123, 456], 789, &[17, 456, 10, 10]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_and_limb_neg_to_out_fail_1() {
    limbs_pos_and_limb_neg_to_out(&mut [], &[], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_and_limb_neg_to_out_fail_2() {
    limbs_pos_and_limb_neg_to_out(&mut [10], &[10, 10], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_and_limb_neg_in_place_fail() {
    limbs_pos_and_limb_neg_in_place(&mut [], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_neg_and_limb_neg_and_limbs_vec_neg_and_limb_neg_in_place() {
    let test = |xs: &[Limb], y: Limb, out: &[Limb]| {
        assert_eq!(limbs_neg_and_limb_neg(xs, y), out);

        let mut xs = xs.to_vec();
        limbs_vec_neg_and_limb_neg_in_place(&mut xs, y);
        assert_eq!(xs, out);
    };
    test(&[0, 2], 3, &[0, 2]);
    test(&[1, 1], 3, &[0xfffffffd, 1]);
    test(&[u32::MAX - 1, 1], 1, &[0, 2]);
    test(&[u32::MAX - 1, u32::MAX], 1, &[0, 0, 1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_neg_and_limb_neg_fail() {
    limbs_neg_and_limb_neg(&[], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_neg_and_limb_neg_to_out() {
    let test = |out_before: &[Limb], xs: &[Limb], y: Limb, carry, out_after: &[Limb]| {
        let mut out = out_before.to_vec();
        assert_eq!(limbs_neg_and_limb_neg_to_out(&mut out, xs, y), carry);
        assert_eq!(out, out_after);
    };
    test(&[0, 0], &[0, 2], 3, false, &[0, 2]);
    test(&[1, 2, 100], &[0, 2, 100], 3, false, &[0, 2, 100]);
    test(&[0, 0], &[1, 1], 3, false, &[0xfffffffd, 1]);
    test(&[0, 0], &[u32::MAX - 1, 1], 1, false, &[0, 2]);
    test(&[0, 0], &[u32::MAX - 1, u32::MAX], 1, true, &[0, 0]);
    test(
        &[1, 2, 100],
        &[u32::MAX - 1, u32::MAX],
        1,
        true,
        &[0, 0, 100],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_neg_and_limb_neg_to_out_fail_1() {
    limbs_neg_and_limb_neg_to_out(&mut [1, 2, 3], &[1, 2, 3, 4], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_neg_and_limb_neg_to_out_fail_2() {
    limbs_neg_and_limb_neg_to_out(&mut [1, 2, 3], &[], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_slice_neg_and_limb_neg_in_place() {
    let test = |xs: &[Limb], y: Limb, carry, xs_after: &[Limb]| {
        let mut xs = xs.to_vec();
        assert_eq!(limbs_slice_neg_and_limb_neg_in_place(&mut xs, y), carry);
        assert_eq!(xs, xs_after);
    };
    test(&[0, 2], 3, false, &[0, 2]);
    test(&[1, 1], 3, false, &[0xfffffffd, 1]);
    test(&[u32::MAX - 1, 1], 1, false, &[0, 2]);
    test(&[u32::MAX - 1, u32::MAX], 1, true, &[0, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_slice_neg_and_limb_neg_in_place_fail() {
    limbs_slice_neg_and_limb_neg_in_place(&mut [], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_vec_neg_and_limb_neg_in_place_fail() {
    let mut xs = vec![];
    limbs_vec_neg_and_limb_neg_in_place(&mut xs, 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_and_pos_neg() {
    let test = |xs, ys, out| {
        assert_eq!(limbs_and_pos_neg(xs, ys), out);
    };
    test(&[2], &[3], vec![0]);
    test(&[1, 1, 1], &[1, 2, 3], vec![1, 1, 0]);
    test(&[6, 7], &[1, 2, 3], vec![6, 5]);
    test(&[1, 2, 3], &[6, 7], vec![0, 0, 3]);
    test(&[100, 101, 102], &[102, 101, 100], vec![0, 0, 2]);
    test(&[0, 0, 1], &[3], vec![0, 0, 1]);
    test(&[3], &[0, 0, 1], vec![]);
    test(&[0, 3, 3], &[0, 0, 3], vec![0, 0, 1]);
    test(&[0, 0, 3], &[0, 3, 3], vec![0, 0, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_and_pos_neg_fail_1() {
    limbs_and_pos_neg(&[0, 0, 0], &[3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_and_pos_neg_fail_2() {
    limbs_and_pos_neg(&[3], &[0, 0, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_and_pos_neg_to_out() {
    let test = |xs, ys, out_before: &[Limb], out_after| {
        let mut out = out_before.to_vec();
        limbs_and_pos_neg_to_out(&mut out, xs, ys);
        assert_eq!(out, out_after);
    };
    test(&[2], &[3], &[10, 10, 10, 10], vec![0, 10, 10, 10]);
    test(&[1, 1, 1], &[1, 2, 3], &[10, 10, 10, 10], vec![1, 1, 0, 10]);
    test(&[6, 7], &[1, 2, 3], &[10, 10, 10, 10], vec![6, 5, 10, 10]);
    test(&[1, 2, 3], &[6, 7], &[10, 10, 10, 10], vec![0, 0, 3, 10]);
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        &[10, 10, 10, 10],
        vec![0, 0, 2, 10],
    );
    test(&[0, 0, 1], &[3], &[10, 10, 10, 10], vec![0, 0, 1, 10]);
    test(&[3], &[0, 0, 1], &[10, 10, 10, 10], vec![0, 10, 10, 10]);
    test(&[0, 3, 3], &[0, 0, 3], &[10, 10, 10, 10], vec![0, 0, 1, 10]);
    test(&[0, 0, 3], &[0, 3, 3], &[10, 10, 10, 10], vec![0, 0, 0, 10]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_and_pos_neg_to_out_fail_1() {
    let mut out = vec![10, 10, 10, 10];
    limbs_and_pos_neg_to_out(&mut out, &[0, 0, 0], &[3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_and_pos_neg_to_out_fail_2() {
    let mut out = vec![10, 10, 10, 10];
    limbs_and_pos_neg_to_out(&mut out, &[3], &[0, 0, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_and_pos_neg_to_out_fail_3() {
    let mut out = vec![10];
    limbs_and_pos_neg_to_out(&mut out, &[6, 7], &[1, 2]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_pos_neg_in_place_left_and_limbs_vec_and_pos_neg_in_place_right() {
    let test = |xs: &[Limb], ys, out| {
        {
            let mut mut_xs = xs.to_vec();
            limbs_and_pos_neg_in_place_left(&mut mut_xs, ys);
            assert_eq!(mut_xs, out);
        }
        {
            let mut mut_ys = ys.to_vec();
            limbs_vec_and_pos_neg_in_place_right(xs, &mut mut_ys);
            assert_eq!(mut_ys, out);
        }
    };
    test(&[2], &[3], vec![0]);
    test(&[1, 1, 1], &[1, 2, 3], vec![1, 1, 0]);
    test(&[6, 7], &[1, 2, 3], vec![6, 5]);
    test(&[1, 2, 3], &[6, 7], vec![0, 0, 3]);
    test(&[100, 101, 102], &[102, 101, 100], vec![0, 0, 2]);
    test(&[0, 0, 1], &[3], vec![0, 0, 1]);
    test(&[3], &[0, 0, 1], vec![0]);
    test(&[0, 3, 3], &[0, 0, 3], vec![0, 0, 1]);
    test(&[0, 0, 3], &[0, 3, 3], vec![0, 0, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_and_pos_neg_in_place_left_fail_1() {
    limbs_and_pos_neg_in_place_left(&mut [0, 0, 0], &[3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_and_pos_neg_in_place_left_fail_2() {
    limbs_and_pos_neg_in_place_left(&mut [3], &[0, 0, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_vec_and_pos_neg_in_place_right_fail_1() {
    let mut ys = vec![3];
    limbs_vec_and_pos_neg_in_place_right(&[0, 0, 0], &mut ys);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_vec_and_pos_neg_in_place_right_fail_2() {
    let mut ys = vec![0, 0, 0];
    limbs_vec_and_pos_neg_in_place_right(&[3], &mut ys);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_slice_and_pos_neg_in_place_right() {
    let test = |xs, ys_before: &[Limb], ys_after| {
        let mut ys = ys_before.to_vec();
        limbs_slice_and_pos_neg_in_place_right(xs, &mut ys);
        assert_eq!(ys, ys_after);
    };
    test(&[2], &[3], vec![0]);
    test(&[1, 1, 1], &[1, 2, 3], vec![1, 1, 0]);
    test(&[6, 7], &[1, 2, 3], vec![6, 5, 3]);
    test(&[1, 2, 3], &[6, 7], vec![0, 0]);
    test(&[100, 101, 102], &[102, 101, 100], vec![0, 0, 2]);
    test(&[0, 0, 1], &[3], vec![0]);
    test(&[3], &[0, 0, 1], vec![0, 0, 0]);
    test(&[0, 3, 3], &[0, 0, 3], vec![0, 0, 1]);
    test(&[0, 0, 3], &[0, 3, 3], vec![0, 0, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_slice_and_pos_neg_in_place_right_fail_1() {
    limbs_slice_and_pos_neg_in_place_right(&[0, 0, 0], &mut [3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_slice_and_pos_neg_in_place_right_fail_2() {
    limbs_slice_and_pos_neg_in_place_right(&[3], &mut [0, 0, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_and_neg_neg_and_limbs_vec_and_neg_neg_in_place_left() {
    let test = |xs, ys, out| {
        assert_eq!(limbs_and_neg_neg(xs, ys), out);

        let mut mut_xs = xs.to_vec();
        limbs_vec_and_neg_neg_in_place_left(&mut mut_xs, ys);
        assert_eq!(mut_xs, out);
    };
    test(&[2], &[3], vec![4]);
    test(&[1, 1, 1], &[1, 2, 3], vec![1, 3, 3]);
    test(&[6, 7], &[1, 2, 3], vec![6, 7, 3]);
    test(&[1, 2, 3], &[6, 7], vec![6, 7, 3]);
    test(&[100, 101, 102], &[102, 101, 100], vec![104, 101, 102]);
    test(&[0, 0, 1], &[3], vec![0, 0, 1]);
    test(&[3], &[0, 0, 1], vec![0, 0, 1]);
    test(&[0, 3, 3], &[0, 0, 3], vec![0, 0, 4]);
    test(&[0, 0, 3], &[0, 3, 3], vec![0, 0, 4]);
    test(&[0, 2, 1], &[0, u32::MAX, u32::MAX], vec![0, 0, 0, 1]);
    test(&[0, 2], &[0, u32::MAX, u32::MAX], vec![0, 0, 0, 1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_and_neg_neg_fail_1() {
    limbs_and_neg_neg(&[0, 0, 0], &[3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_and_neg_neg_fail_2() {
    limbs_and_neg_neg(&[3], &[0, 0, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_vec_and_neg_neg_in_place_left_fail_1() {
    limbs_vec_and_neg_neg_in_place_left(&mut vec![0, 0, 0], &[3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_vec_and_neg_neg_in_place_left_fail_2() {
    limbs_vec_and_neg_neg_in_place_left(&mut vec![3], &[0, 0, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_and_neg_neg_to_out() {
    let test = |xs, ys, out_before: &[Limb], b, out_after| {
        let mut out = out_before.to_vec();
        assert_eq!(limbs_and_neg_neg_to_out(&mut out, xs, ys), b);
        assert_eq!(out, out_after);
    };
    test(&[2], &[3], &[10, 10, 10, 10], true, vec![4, 10, 10, 10]);
    test(
        &[1, 1, 1],
        &[1, 2, 3],
        &[10, 10, 10, 10],
        true,
        vec![1, 3, 3, 10],
    );
    test(
        &[6, 7],
        &[1, 2, 3],
        &[10, 10, 10, 10],
        true,
        vec![6, 7, 3, 10],
    );
    test(
        &[1, 2, 3],
        &[6, 7],
        &[10, 10, 10, 10],
        true,
        vec![6, 7, 3, 10],
    );
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        &[10, 10, 10, 10],
        true,
        vec![104, 101, 102, 10],
    );
    test(&[0, 0, 1], &[3], &[10, 10, 10, 10], true, vec![0, 0, 1, 10]);
    test(&[3], &[0, 0, 1], &[10, 10, 10, 10], true, vec![0, 0, 1, 10]);
    test(
        &[0, 3, 3],
        &[0, 0, 3],
        &[10, 10, 10, 10],
        true,
        vec![0, 0, 4, 10],
    );
    test(
        &[0, 0, 3],
        &[0, 3, 3],
        &[10, 10, 10, 10],
        true,
        vec![0, 0, 4, 10],
    );
    test(
        &[0, 2],
        &[0, u32::MAX],
        &[10, 10, 10, 10],
        false,
        vec![0, 0, 10, 10],
    );
    test(
        &[0, 2, 1],
        &[0, u32::MAX, u32::MAX],
        &[10, 10, 10, 10],
        false,
        vec![0, 0, 0, 10],
    );
    test(
        &[0, 2],
        &[0, u32::MAX, u32::MAX],
        &[10, 10, 10, 10],
        false,
        vec![0, 0, 0, 10],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_and_neg_neg_to_out_fail_1() {
    let mut out = vec![10, 10, 10, 10];
    limbs_and_neg_neg_to_out(&mut out, &[0, 0, 0], &[3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_and_neg_neg_to_out_fail_2() {
    let mut out = vec![10, 10, 10, 10];
    limbs_and_neg_neg_to_out(&mut out, &[3], &[0, 0, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_and_neg_neg_to_out_fail_3() {
    let mut out = vec![10, 10];
    limbs_and_neg_neg_to_out(&mut out, &[6, 7, 8], &[1, 2]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_and_neg_neg_to_out_fail_4() {
    let mut out = vec![10, 10];
    limbs_and_neg_neg_to_out(&mut out, &[6, 7], &[1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_slice_and_neg_neg_in_place_left() {
    let test = |xs_before: &[Limb], ys, b, xs_after| {
        let mut xs = xs_before.to_vec();
        assert_eq!(limbs_slice_and_neg_neg_in_place_left(&mut xs, ys), b);
        assert_eq!(xs, xs_after);
    };
    test(&[2], &[3], true, vec![4]);
    test(&[1, 1, 1], &[1, 2, 3], true, vec![1, 3, 3]);
    test(&[1, 2, 3], &[6, 7], true, vec![6, 7, 3]);
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        true,
        vec![104, 101, 102],
    );
    test(&[0, 0, 1], &[3], true, vec![0, 0, 1]);
    test(&[0, 3, 3], &[0, 0, 3], true, vec![0, 0, 4]);
    test(&[0, 0, 3], &[0, 3, 3], true, vec![0, 0, 4]);
    test(&[0, 2], &[0, u32::MAX], false, vec![0, 0]);
    test(&[0, 2, 1], &[0, u32::MAX, u32::MAX], false, vec![0, 0, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_slice_and_neg_neg_in_place_left_fail_1() {
    limbs_slice_and_neg_neg_in_place_left(&mut [0, 0, 0], &[3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_slice_and_neg_neg_in_place_left_fail_2() {
    limbs_slice_and_neg_neg_in_place_left(&mut [0, 0, 1], &[0, 0, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_slice_and_neg_neg_in_place_left_fail_3() {
    limbs_slice_and_neg_neg_in_place_left(&mut [6, 7], &[1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_slice_and_neg_neg_in_place_either() {
    let test = |xs_before: &[Limb], ys_before: &[Limb], p, xs_after, ys_after| {
        let mut xs = xs_before.to_vec();
        let mut ys = ys_before.to_vec();
        assert_eq!(limbs_slice_and_neg_neg_in_place_either(&mut xs, &mut ys), p);
        assert_eq!(xs, xs_after);
        assert_eq!(ys, ys_after);
    };
    test(&[2], &[3], (false, true), vec![4], vec![3]);
    test(
        &[1, 1, 1],
        &[1, 2, 3],
        (false, true),
        vec![1, 3, 3],
        vec![1, 2, 3],
    );
    test(
        &[1, 2, 3],
        &[6, 7],
        (false, true),
        vec![6, 7, 3],
        vec![6, 7],
    );
    test(&[6, 7], &[1, 2, 3], (true, true), vec![6, 7], vec![6, 7, 3]);
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        (false, true),
        vec![104, 101, 102],
        vec![102, 101, 100],
    );
    test(&[0, 0, 1], &[3], (false, true), vec![0, 0, 1], vec![3]);
    test(&[3], &[0, 0, 1], (true, true), vec![3], vec![0, 0, 1]);
    test(
        &[0, 3, 3],
        &[0, 0, 3],
        (false, true),
        vec![0, 0, 4],
        vec![0, 0, 3],
    );
    test(
        &[0, 0, 3],
        &[0, 3, 3],
        (false, true),
        vec![0, 0, 4],
        vec![0, 3, 3],
    );
    test(
        &[0, 2],
        &[0, u32::MAX],
        (false, false),
        vec![0, 0],
        vec![0, u32::MAX],
    );
    test(
        &[0, 2, 1],
        &[0, u32::MAX, u32::MAX],
        (false, false),
        vec![0, 0, 0],
        vec![0, u32::MAX, u32::MAX],
    );
    test(
        &[0, 2],
        &[0, u32::MAX, u32::MAX],
        (true, false),
        vec![0, 2],
        vec![0, 0, 0],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_slice_and_neg_neg_in_place_either_fail_1() {
    limbs_slice_and_neg_neg_in_place_either(&mut [0, 0, 0], &mut [3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_slice_and_neg_neg_in_place_either_fail_2() {
    limbs_slice_and_neg_neg_in_place_either(&mut [3], &mut [0, 0, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_vec_and_neg_neg_in_place_either() {
    let test = |xs_before: &[Limb], ys_before: &[Limb], b, xs_after, ys_after| {
        let mut xs = xs_before.to_vec();
        let mut ys = ys_before.to_vec();
        assert_eq!(limbs_vec_and_neg_neg_in_place_either(&mut xs, &mut ys), b);
        assert_eq!(xs, xs_after);
        assert_eq!(ys, ys_after);
    };
    test(&[2], &[3], false, vec![4], vec![3]);
    test(&[1, 1, 1], &[1, 2, 3], false, vec![1, 3, 3], vec![1, 2, 3]);
    test(&[1, 2, 3], &[6, 7], false, vec![6, 7, 3], vec![6, 7]);
    test(&[6, 7], &[1, 2, 3], true, vec![6, 7], vec![6, 7, 3]);
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        false,
        vec![104, 101, 102],
        vec![102, 101, 100],
    );
    test(&[0, 0, 1], &[3], false, vec![0, 0, 1], vec![3]);
    test(&[3], &[0, 0, 1], true, vec![3], vec![0, 0, 1]);
    test(&[0, 3, 3], &[0, 0, 3], false, vec![0, 0, 4], vec![0, 0, 3]);
    test(&[0, 0, 3], &[0, 3, 3], false, vec![0, 0, 4], vec![0, 3, 3]);
    test(
        &[0, 2],
        &[0, u32::MAX],
        false,
        vec![0, 0, 1],
        vec![0, u32::MAX],
    );
    test(
        &[0, 2, 1],
        &[0, u32::MAX, u32::MAX],
        false,
        vec![0, 0, 0, 1],
        vec![0, u32::MAX, u32::MAX],
    );
    test(
        &[0, 2],
        &[0, u32::MAX, u32::MAX],
        true,
        vec![0, 2],
        vec![0, 0, 0, 1],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_vec_and_neg_neg_in_place_either_fail_1() {
    limbs_vec_and_neg_neg_in_place_either(&mut vec![0, 0, 0], &mut vec![3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_vec_and_neg_neg_in_place_either_fail_2() {
    limbs_vec_and_neg_neg_in_place_either(&mut vec![3], &mut vec![0, 0, 0]);
}

#[test]
fn test_and() {
    let test = |s, t, out| {
        let u = Integer::from_str(s).unwrap();
        let v = Integer::from_str(t).unwrap();

        let mut n = u.clone();
        n &= v.clone();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = u.clone();
        n &= &v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone() & v.clone();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &u & v.clone();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone() & &v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &u & &v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        assert_eq!(integer_and_alt_1(&u, &v).to_string(), out);
        assert_eq!(integer_and_alt_2(&u, &v).to_string(), out);

        let n = rug::Integer::from_str(s).unwrap() & rug::Integer::from_str(t).unwrap();
        assert_eq!(n.to_string(), out);
    };
    test("0", "0", "0");
    test("0", "123", "0");
    test("123", "0", "0");
    test("123", "456", "72");
    test("1000000000000", "123", "0");
    test("123", "1000000000000", "0");
    test("1000000000000", "999999999999", "999999995904");
    test("12345678987654321", "314159265358979", "312331665941633");
    test("0", "-123", "0");
    test("123", "-456", "56");
    test("1000000000000", "-123", "1000000000000");
    test("123", "-1000000000000", "0");
    test("1000000000000", "-999999999999", "4096");
    test("12345678987654321", "-314159265358979", "12033347321712689");
    test("-123", "0", "0");
    test("-123", "456", "384");
    test("-1000000000000", "123", "0");
    test("-123", "1000000000000", "1000000000000");
    test("-1000000000000", "999999999999", "0");
    test("-12345678987654321", "314159265358979", "1827599417347");
    test("-123", "-456", "-512");
    test("-1000000000000", "-123", "-1000000000000");
    test("-123", "-1000000000000", "-1000000000000");
    test("-1000000000000", "-999999999999", "-1000000000000");
    test(
        "-12345678987654321",
        "-314159265358979",
        "-12347506587071667",
    );

    test(
        "-18446744073708507135",
        "-9007061819981696",
        "-18446744073709551616",
    );
    test("-18446744073708507135", "-4194176", "-18446744073709551616");
    test("-4194176", "-18446744073708507135", "-18446744073709551616");
    test(
        "3332140978726732268209104861552",
        "-478178031043645514337313657924474082957368",
        "2539024739207132029580719268160",
    );
    test(
        "-478178031043645514337313657924474082957368",
        "3332140978726732268209104861552",
        "2539024739207132029580719268160",
    );
}

#[test]
fn limbs_pos_and_limb_neg_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_15().test_properties_with_config(&config, |(xs, y)| {
        let out = limbs_pos_and_limb_neg(&xs, y);
        let n = Integer::from(Natural::from_owned_limbs_asc(xs))
            & Integer::from_owned_twos_complement_limbs_asc(vec![y, Limb::MAX]);
        assert_eq!(Natural::from_owned_limbs_asc(out), Natural::exact_from(n));
    });
}

#[test]
fn limbs_pos_and_limb_neg_to_out_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_4().test_properties_with_config(
        &config,
        |(mut out, xs, y)| {
            let old_out = out.clone();
            limbs_pos_and_limb_neg_to_out(&mut out, &xs, y);
            let len = xs.len();
            let n = Integer::from(Natural::from_owned_limbs_asc(xs))
                & Integer::from_owned_twos_complement_limbs_asc(vec![y, Limb::MAX]);
            let mut out_alt = Natural::exact_from(n).into_limbs_asc();
            out_alt.resize(len, 0);
            assert_eq!(out_alt, &out[..len]);
            assert_eq!(&out[len..], &old_out[len..]);
        },
    );
}

#[test]
fn limbs_pos_and_limb_neg_in_place_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_15().test_properties_with_config(&config, |(mut xs, y)| {
        let mut n = Integer::from(Natural::from_limbs_asc(&xs));
        limbs_pos_and_limb_neg_in_place(&mut xs, y);
        n &= Integer::from_owned_twos_complement_limbs_asc(vec![y, Limb::MAX]);
        assert_eq!(Natural::from_owned_limbs_asc(xs), Natural::exact_from(n));
    });
}

#[test]
fn limbs_neg_and_limb_neg_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_18().test_properties_with_config(&config, |(xs, y)| {
        let out = limbs_neg_and_limb_neg(&xs, y);
        let n = -Natural::from_owned_limbs_asc(xs)
            & Integer::from_owned_twos_complement_limbs_asc(vec![y, Limb::MAX]);
        assert_eq!(Natural::from_owned_limbs_asc(out), Natural::exact_from(-n));
    });
}

#[test]
fn limbs_neg_and_limb_neg_to_out_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_5().test_properties_with_config(
        &config,
        |(mut out, xs, y)| {
            let old_out = out.clone();
            let carry = limbs_neg_and_limb_neg_to_out(&mut out, &xs, y);
            let len = xs.len();
            let n = -Natural::from_owned_limbs_asc(xs)
                & Integer::from_owned_twos_complement_limbs_asc(vec![y, Limb::MAX]);
            let mut out_alt = Natural::exact_from(-n).into_limbs_asc();
            assert_eq!(carry, out_alt.len() == len + 1);
            out_alt.resize(len, 0);
            assert_eq!(out_alt, &out[..len]);
            assert_eq!(&out[len..], &old_out[len..]);
        },
    );
}

#[test]
fn limbs_slice_neg_and_limb_neg_in_place_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_18().test_properties_with_config(&config, |(mut xs, y)| {
        let old_xs = xs.clone();
        limbs_slice_neg_and_limb_neg_in_place(&mut xs, y);
        let n = -Natural::from_owned_limbs_asc(old_xs)
            & Integer::from_owned_twos_complement_limbs_asc(vec![y, Limb::MAX]);
        let mut expected_xs = Natural::exact_from(-n).into_limbs_asc();
        expected_xs.resize(xs.len(), 0);
        assert_eq!(xs, expected_xs);
    });
}

#[test]
fn limbs_vec_neg_and_limb_neg_in_place_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_18().test_properties_with_config(&config, |(mut xs, y)| {
        let n = Natural::from_limbs_asc(&xs);
        limbs_vec_neg_and_limb_neg_in_place(&mut xs, y);
        let n = -n & Integer::from_owned_twos_complement_limbs_asc(vec![y, Limb::MAX]);
        assert_eq!(Natural::from_owned_limbs_asc(xs), Natural::exact_from(-n));
    });
}

#[test]
fn limbs_and_pos_neg_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen_var_8().test_properties_with_config(&config, |(xs, ys)| {
        assert_eq!(
            Natural::from_owned_limbs_asc(limbs_and_pos_neg(&xs, &ys)),
            Integer::from(Natural::from_owned_limbs_asc(xs)) & -Natural::from_owned_limbs_asc(ys)
        );
    });
}

#[test]
fn limbs_and_pos_neg_to_out_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_triple_gen_var_33().test_properties_with_config(&config, |(mut out, xs, ys)| {
        let out_old = out.clone();
        limbs_and_pos_neg_to_out(&mut out, &xs, &ys);
        let len = xs.len();
        assert_eq!(
            Natural::from_limbs_asc(&out[..len]),
            Integer::from(Natural::from_owned_limbs_asc(xs)) & -Natural::from_owned_limbs_asc(ys)
        );
        assert_eq!(&out[len..], &out_old[len..]);
    });
}

#[test]
fn limbs_and_pos_neg_in_place_left_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen_var_8().test_properties_with_config(&config, |(mut xs, ys)| {
        let xs_old = xs.clone();
        limbs_and_pos_neg_in_place_left(&mut xs, &ys);
        assert_eq!(
            Natural::from_owned_limbs_asc(xs),
            Integer::from(Natural::from_owned_limbs_asc(xs_old))
                & -Natural::from_owned_limbs_asc(ys)
        );
    });
}

#[test]
fn limbs_slice_and_pos_neg_in_place_right_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen_var_8().test_properties_with_config(&config, |(xs, mut ys)| {
        let ys_old = ys.clone();
        limbs_slice_and_pos_neg_in_place_right(&xs, &mut ys);
        let len = min(xs.len(), ys.len());
        let result = Integer::from(Natural::from_owned_limbs_asc(xs))
            & -Natural::from_owned_limbs_asc(ys_old);
        let mut expected_ys = Natural::exact_from(result).into_limbs_asc();
        expected_ys.resize(len, 0);
        assert_eq!(&ys[..len], expected_ys.as_slice());
    });
}

#[test]
fn limbs_vec_and_pos_neg_in_place_right_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen_var_8().test_properties_with_config(&config, |(xs, mut ys)| {
        let ys_old = ys.clone();
        limbs_vec_and_pos_neg_in_place_right(&xs, &mut ys);
        let len = xs.len();
        let result = Integer::from(Natural::from_owned_limbs_asc(xs))
            & -Natural::from_owned_limbs_asc(ys_old);
        let mut expected_limbs = Natural::exact_from(result).into_limbs_asc();
        expected_limbs.resize(len, 0);
        ys.resize(len, 0);
        assert_eq!(ys, expected_limbs);
    });
}

#[test]
fn limbs_and_neg_neg_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen_var_8().test_properties_with_config(&config, |(xs, ys)| {
        assert_eq!(
            -Natural::from_owned_limbs_asc(limbs_and_neg_neg(&xs, &ys)),
            -Natural::from_owned_limbs_asc(xs) & -Natural::from_owned_limbs_asc(ys)
        );
    });
}

#[test]
fn limbs_and_neg_neg_to_out_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_triple_gen_var_34().test_properties_with_config(&config, |(mut out, xs, ys)| {
        let out_old = out.clone();
        let b = limbs_and_neg_neg_to_out(&mut out, &xs, &ys);
        let len = max(xs.len(), ys.len());
        let result = Natural::exact_from(
            -(-Natural::from_owned_limbs_asc(xs) & -Natural::from_owned_limbs_asc(ys)),
        );
        let mut expected_out = result.to_limbs_asc();
        expected_out.resize(len, 0);
        assert_eq!(&out[..len], expected_out.as_slice());
        assert_eq!(b, Natural::from_owned_limbs_asc(expected_out) == result);
        assert_eq!(&out[len..], &out_old[len..]);
    });
}

#[test]
fn limbs_slice_and_neg_neg_in_place_left_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen_var_9().test_properties_with_config(&config, |(mut xs, ys)| {
        let xs_old = xs.clone();
        let b = limbs_slice_and_neg_neg_in_place_left(&mut xs, &ys);
        let len = xs_old.len();
        let result = Natural::exact_from(
            -(-Natural::from_owned_limbs_asc(xs_old) & -Natural::from_owned_limbs_asc(ys)),
        );
        let mut expected_xs = result.to_limbs_asc();
        expected_xs.resize(len, 0);
        assert_eq!(xs, expected_xs.as_slice());
        assert_eq!(b, Natural::from_owned_limbs_asc(expected_xs) == result);
    });
}

#[test]
fn limbs_vec_and_neg_neg_in_place_left_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen_var_8().test_properties_with_config(&config, |(mut xs, ys)| {
        let xs_old = xs.clone();
        limbs_vec_and_neg_neg_in_place_left(&mut xs, &ys);
        assert_eq!(
            -Natural::from_owned_limbs_asc(xs),
            -Natural::from_owned_limbs_asc(xs_old) & -Natural::from_owned_limbs_asc(ys)
        );
    });
}

#[test]
fn limbs_slice_and_neg_neg_in_place_either_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen_var_8().test_properties_with_config(&config, |(mut xs, mut ys)| {
        let xs_old = xs.clone();
        let ys_old = ys.clone();
        let (right, b) = limbs_slice_and_neg_neg_in_place_either(&mut xs, &mut ys);
        let len = max(xs_old.len(), ys_old.len());
        let result = Natural::exact_from(
            -(-Natural::from_limbs_asc(&xs_old) & -Natural::from_limbs_asc(&ys_old)),
        );
        let mut expected_limbs = result.to_limbs_asc();
        expected_limbs.resize(len, 0);
        assert_eq!(b, Natural::from_limbs_asc(&expected_limbs) == result);
        if right {
            assert_eq!(ys, expected_limbs.as_slice());
            assert_eq!(xs, xs_old);
        } else {
            assert_eq!(xs, expected_limbs.as_slice());
            assert_eq!(ys, ys_old);
        }
    });
}

#[test]
fn limbs_vec_and_neg_neg_in_place_either_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen_var_8().test_properties_with_config(&config, |(mut xs, mut ys)| {
        let xs_old = xs.clone();
        let ys_old = ys.clone();
        let right = limbs_vec_and_neg_neg_in_place_either(&mut xs, &mut ys);
        let expected = -Natural::from_limbs_asc(&xs_old) & -Natural::from_limbs_asc(&ys_old);
        if right {
            assert_eq!(xs, xs_old);
            assert_eq!(-Natural::from_owned_limbs_asc(ys), expected);
        } else {
            assert_eq!(-Natural::from_owned_limbs_asc(xs), expected);
            assert_eq!(ys, ys_old);
        }
    });
}

#[allow(clippy::eq_op)]
#[test]
fn and_properties() {
    integer_pair_gen().test_properties(|(x, y)| {
        let result_val_val = x.clone() & y.clone();
        let result_val_ref = x.clone() & &y;
        let result_ref_val = &x & y.clone();
        let result = &x & &y;
        assert!(result_val_val.is_valid());
        assert!(result_val_ref.is_valid());
        assert!(result_ref_val.is_valid());
        assert!(result.is_valid());
        assert_eq!(result_val_val, result);
        assert_eq!(result_val_ref, result);
        assert_eq!(result_ref_val, result);

        let mut mut_x = x.clone();
        mut_x &= y.clone();
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, result);
        let mut mut_x = x.clone();
        mut_x &= &y;
        assert_eq!(mut_x, result);
        assert!(mut_x.is_valid());

        let mut mut_x = rug::Integer::from(&x);
        mut_x &= rug::Integer::from(&y);
        assert_eq!(Integer::from(&mut_x), result);

        assert_eq!(
            Integer::from(&(rug::Integer::from(&x) & rug::Integer::from(&y))),
            result
        );

        assert_eq!(integer_and_alt_1(&x, &y), result);
        assert_eq!(integer_and_alt_2(&x, &y), result);

        assert_eq!(&y & &x, result);
        assert_eq!(&result & &x, result);
        assert_eq!(&result & &y, result);
        assert_eq!(!(!x | !y), result);
    });

    integer_gen().test_properties(|x| {
        assert_eq!(&x & Integer::ZERO, 0);
        assert_eq!(Integer::ZERO & &x, 0);
        assert_eq!(&x & &x, x);
        assert_eq!(&x & !&x, 0);
    });

    integer_triple_gen().test_properties(|(x, y, z)| {
        assert_eq!((&x & &y) & &z, x & (y & z));
    });

    signed_pair_gen::<SignedLimb>().test_properties(|(i, j)| {
        assert_eq!(Integer::from(i) & Integer::from(j), i & j);
    });

    natural_pair_gen().test_properties(|(x, y)| {
        assert_eq!(Integer::from(&x) & Integer::from(&y), x & y);
    });
}
