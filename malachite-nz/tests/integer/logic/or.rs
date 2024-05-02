// Copyright © 2024 Mikhail Hogrefe
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
    signed_pair_gen, unsigned_vec_pair_gen_var_8, unsigned_vec_triple_gen_var_33,
    unsigned_vec_triple_gen_var_35, unsigned_vec_unsigned_pair_gen_var_18,
    unsigned_vec_unsigned_pair_gen_var_22, unsigned_vec_unsigned_vec_unsigned_triple_gen_var_5,
};
use malachite_nz::integer::logic::or::{
    limbs_neg_or_limb, limbs_neg_or_limb_in_place, limbs_neg_or_limb_to_out, limbs_neg_or_neg_limb,
    limbs_or_neg_neg, limbs_or_neg_neg_in_place_either, limbs_or_neg_neg_to_out, limbs_or_pos_neg,
    limbs_or_pos_neg_in_place_right, limbs_or_pos_neg_to_out, limbs_pos_or_neg_limb,
    limbs_slice_or_neg_neg_in_place_left, limbs_slice_or_pos_neg_in_place_left,
    limbs_vec_or_neg_neg_in_place_left, limbs_vec_or_pos_neg_in_place_left,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::{Limb, SignedLimb};
use malachite_nz::test_util::generators::{
    integer_gen, integer_pair_gen, integer_triple_gen, natural_pair_gen,
};
use malachite_nz::test_util::integer::logic::or::{integer_or_alt_1, integer_or_alt_2};
use rug;
use std::cmp::min;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_neg_or_limb_and_limbs_neg_or_limb_in_place() {
    let test = |xs: &[Limb], y: Limb, out: &[Limb]| {
        assert_eq!(limbs_neg_or_limb(xs, y), out);

        let mut xs = xs.to_vec();
        limbs_neg_or_limb_in_place(&mut xs, y);
        assert_eq!(xs, out);
    };
    test(&[6, 7], 0, &[6, 7]);
    test(&[6, 7], 2, &[6, 7]);
    test(&[100, 101, 102], 10, &[98, 101, 102]);
    test(&[123, 456], 789, &[107, 456]);
    test(&[0, 0, 456], 789, &[0xfffffceb, Limb::MAX, 455]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_neg_or_limb_in_place_fail() {
    limbs_neg_or_limb_in_place(&mut [0, 0, 0], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_neg_or_limb_fail() {
    limbs_neg_or_limb(&[0, 0, 0], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_neg_or_limb_to_out() {
    let test = |out_before: &[Limb], xs: &[Limb], y: Limb, out_after: &[Limb]| {
        let mut out = out_before.to_vec();
        limbs_neg_or_limb_to_out(&mut out, xs, y);
        assert_eq!(out, out_after);
    };
    test(&[10, 10, 10, 10], &[6, 7], 0, &[6, 7, 10, 10]);
    test(&[10, 10, 10, 10], &[6, 7], 2, &[6, 7, 10, 10]);
    test(&[10, 10, 10, 10], &[100, 101, 102], 10, &[98, 101, 102, 10]);
    test(&[10, 10, 10, 10], &[123, 456], 789, &[107, 456, 10, 10]);
    test(
        &[10, 10, 10, 10],
        &[0, 0, 456],
        789,
        &[0xfffffceb, Limb::MAX, 455, 10],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_neg_or_limb_to_out_fail_1() {
    limbs_neg_or_limb_to_out(&mut [10, 10], &[0, 0], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_neg_or_limb_to_out_fail_2() {
    limbs_neg_or_limb_to_out(&mut [10], &[10, 10], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_pos_or_neg_limb() {
    let test = |xs: &[Limb], y: Limb, out: Limb| {
        assert_eq!(limbs_pos_or_neg_limb(xs, y), out);
    };
    test(&[6, 7], 3, 0xfffffff9);
    test(&[100, 101, 102], 10, 0xffffff92);
    test(&[0, 0, 1], 100, 0xffffff9c);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
const fn limbs_pos_or_neg_limb_fail() {
    limbs_pos_or_neg_limb(&[], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_neg_or_neg_limb() {
    let test = |xs: &[Limb], y: Limb, out: Limb| {
        assert_eq!(limbs_neg_or_neg_limb(xs, y), out);
    };
    test(&[6, 7], 3, 5);
    test(&[100, 101, 102], 10, 98);
    test(&[0, 0, 1], 100, 0xffffff9c);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
const fn limbs_neg_or_neg_limb_fail() {
    limbs_neg_or_neg_limb(&[], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_or_pos_neg_limbs_vec_or_pos_neg_in_place_left_and_limbs_or_pos_neg_in_place_right() {
    let test = |xs_before, ys_before, out| {
        assert_eq!(limbs_or_pos_neg(xs_before, ys_before), out);

        let mut xs = xs_before.to_vec();
        limbs_vec_or_pos_neg_in_place_left(&mut xs, ys_before);
        assert_eq!(xs, out);

        let mut ys = ys_before.to_vec();
        limbs_or_pos_neg_in_place_right(xs_before, &mut ys);
        assert_eq!(ys, out);
    };
    test(&[2], &[3], vec![1]);
    test(&[1, 1, 1], &[1, 2, 3], vec![1, 2, 2]);
    test(&[6, 7], &[1, 2, 3], vec![1, 0, 3]);
    test(&[1, 2, 3], &[6, 7], vec![5, 5]);
    test(&[100, 101, 102], &[102, 101, 100], vec![2, 0, 0]);
    test(&[0, 0, 1], &[3], vec![3]);
    test(&[3], &[0, 0, 1], vec![0xfffffffd, Limb::MAX, 0]);
    test(&[0, 3, 3], &[0, 0, 3], vec![0, 0xfffffffd, 0]);
    test(&[0, 0, 3], &[0, 3, 3], vec![0, 3, 0]);
    test(&[0, 3], &[0, 0, 3], vec![0, 0xfffffffd, 2]);
    test(&[0, 0, 3], &[0, 3], vec![0, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_or_pos_neg_fail_1() {
    limbs_or_pos_neg(&[0, 0, 0], &[3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_or_pos_neg_fail_2() {
    limbs_or_pos_neg(&[3], &[0, 0, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_or_pos_neg_to_out() {
    let test = |xs, ys, out_before: &[Limb], out_after| {
        let mut out = out_before.to_vec();
        limbs_or_pos_neg_to_out(&mut out, xs, ys);
        assert_eq!(out, out_after);
    };
    test(&[2], &[3], &[10, 10, 10, 10], vec![1, 10, 10, 10]);
    test(&[1, 1, 1], &[1, 2, 3], &[10, 10, 10, 10], vec![1, 2, 2, 10]);
    test(&[6, 7], &[1, 2, 3], &[10, 10, 10, 10], vec![1, 0, 3, 10]);
    test(&[1, 2, 3], &[6, 7], &[10, 10, 10, 10], vec![5, 5, 10, 10]);
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        &[10, 10, 10, 10],
        vec![2, 0, 0, 10],
    );
    test(&[0, 0, 1], &[3], &[10, 10, 10, 10], vec![3, 10, 10, 10]);
    test(
        &[3],
        &[0, 0, 1],
        &[10, 10, 10, 10],
        vec![0xfffffffd, Limb::MAX, 0, 10],
    );
    test(
        &[0, 3, 3],
        &[0, 0, 3],
        &[10, 10, 10, 10],
        vec![0, 0xfffffffd, 0, 10],
    );
    test(&[0, 0, 3], &[0, 3, 3], &[10, 10, 10, 10], vec![0, 3, 0, 10]);
    test(
        &[0, 3],
        &[0, 0, 3],
        &[10, 10, 10, 10],
        vec![0, 0xfffffffd, 2, 10],
    );
    test(&[0, 0, 3], &[0, 3], &[10, 10, 10, 10], vec![0, 3, 10, 10]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_or_pos_neg_to_out_fail_1() {
    let mut out = vec![10, 10, 10, 10];
    limbs_or_pos_neg_to_out(&mut out, &[0, 0, 0], &[3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_or_pos_neg_to_out_fail_2() {
    let mut out = vec![10, 10, 10, 10];
    limbs_or_pos_neg_to_out(&mut out, &[3], &[0, 0, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_or_pos_neg_to_out_fail_3() {
    let mut out = vec![10];
    limbs_or_pos_neg_to_out(&mut out, &[6, 7], &[1, 2]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_slice_or_pos_neg_in_place_left() {
    let test = |xs_before: &[Limb], ys, b, xs_after| {
        let mut xs = xs_before.to_vec();
        assert_eq!(limbs_slice_or_pos_neg_in_place_left(&mut xs, ys), b);
        assert_eq!(xs, xs_after);
    };
    test(&[2], &[3], false, vec![1]);
    test(&[1, 1, 1], &[1, 2, 3], false, vec![1, 2, 2]);
    test(&[6, 7], &[1, 2, 3], true, vec![1, 0]);
    test(&[1, 2, 3], &[6, 7], false, vec![5, 5, 0]);
    test(&[100, 101, 102], &[102, 101, 100], false, vec![2, 0, 0]);
    test(&[0, 0, 1], &[3], false, vec![3, 0, 0]);
    test(&[3], &[0, 0, 1], true, vec![0xfffffffd]);
    test(&[0, 3, 3], &[0, 0, 3], false, vec![0, 0xfffffffd, 0]);
    test(&[0, 0, 3], &[0, 3, 3], false, vec![0, 3, 0]);
    test(&[0, 3], &[0, 0, 3], true, vec![0, 0xfffffffd]);
    test(&[0, 0, 3], &[0, 3], false, vec![0, 3, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_slice_or_pos_neg_in_place_left_fail_1() {
    limbs_slice_or_pos_neg_in_place_left(&mut [0, 0, 0], &[3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_slice_or_pos_neg_in_place_left_fail_2() {
    limbs_slice_or_pos_neg_in_place_left(&mut [3], &[0, 0, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_vec_or_pos_neg_in_place_left_fail_1() {
    limbs_vec_or_pos_neg_in_place_left(&mut vec![0, 0, 0], &[3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_vec_or_pos_neg_in_place_left_fail_2() {
    limbs_vec_or_pos_neg_in_place_left(&mut vec![3], &[0, 0, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_or_pos_neg_in_place_right_fail_1() {
    limbs_or_pos_neg_in_place_right(&[0, 0, 0], &mut [3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_or_pos_neg_in_place_right_fail_2() {
    limbs_or_pos_neg_in_place_right(&[3], &mut [0, 0, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_or_neg_neg_and_limbs_vec_or_neg_neg_in_place_left() {
    let test = |xs_before, ys, out| {
        assert_eq!(limbs_or_neg_neg(xs_before, ys), out);

        let mut xs = xs_before.to_vec();
        limbs_vec_or_neg_neg_in_place_left(&mut xs, ys);
        assert_eq!(xs, out);
    };
    test(&[2], &[3], vec![1]);
    test(&[1, 1, 1], &[1, 2, 3], vec![1, 0, 1]);
    test(&[6, 7], &[1, 2, 3], vec![1, 2]);
    test(&[1, 2, 3], &[6, 7], vec![1, 2]);
    test(&[100, 101, 102], &[102, 101, 100], vec![98, 101, 100]);
    test(&[0, 0, 1], &[3], vec![3]);
    test(&[3], &[0, 0, 1], vec![3]);
    test(&[0, 3, 3], &[0, 0, 3], vec![0, 3, 2]);
    test(&[0, 0, 3], &[0, 3, 3], vec![0, 3, 2]);
    test(&[0, 3], &[0, 0, 3], vec![0, 3]);
    test(&[0, 0, 3], &[0, 3], vec![0, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_or_neg_neg_fail_1() {
    limbs_or_neg_neg(&[0, 0, 0], &[3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_or_neg_neg_fail_2() {
    limbs_or_neg_neg(&[3], &[0, 0, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_or_neg_neg_to_out() {
    let test = |xs, ys, out_before: &[Limb], out_after| {
        let mut out = out_before.to_vec();
        limbs_or_neg_neg_to_out(&mut out, xs, ys);
        assert_eq!(out, out_after);
    };
    test(&[2], &[3], &[10, 10, 10, 10], vec![1, 10, 10, 10]);
    test(&[1, 1, 1], &[1, 2, 3], &[10, 10, 10, 10], vec![1, 0, 1, 10]);
    test(&[6, 7], &[1, 2, 3], &[10, 10, 10, 10], vec![1, 2, 10, 10]);
    test(&[1, 2, 3], &[6, 7], &[10, 10, 10, 10], vec![1, 2, 10, 10]);
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        &[10, 10, 10, 10],
        vec![98, 101, 100, 10],
    );
    test(&[0, 0, 1], &[3], &[10, 10, 10, 10], vec![3, 10, 10, 10]);
    test(&[3], &[0, 0, 1], &[10, 10, 10, 10], vec![3, 10, 10, 10]);
    test(&[0, 3, 3], &[0, 0, 3], &[10, 10, 10, 10], vec![0, 3, 2, 10]);
    test(&[0, 0, 3], &[0, 3, 3], &[10, 10, 10, 10], vec![0, 3, 2, 10]);
    test(&[0, 3], &[0, 0, 3], &[10, 10, 10, 10], vec![0, 3, 10, 10]);
    test(&[0, 0, 3], &[0, 3], &[10, 10, 10, 10], vec![0, 3, 10, 10]);

    test(&[1, 2, 3], &[6, 7], &[10, 10], vec![1, 2]);
    test(&[6, 7], &[1, 2, 3], &[10, 10], vec![1, 2]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_or_neg_neg_to_out_fail_1() {
    let mut out = vec![10, 10, 10, 10];
    limbs_or_neg_neg_to_out(&mut out, &[0, 0, 0], &[3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_or_neg_neg_to_out_fail_2() {
    let mut out = vec![10, 10, 10, 10];
    limbs_or_neg_neg_to_out(&mut out, &[3], &[0, 0, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_or_neg_neg_to_out_fail_3() {
    let mut out = vec![10];
    limbs_or_neg_neg_to_out(&mut out, &[6, 7, 8], &[1, 2]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_slice_or_neg_neg_in_place_left() {
    let test = |xs_before: &[Limb], ys, xs_after| {
        let mut xs = xs_before.to_vec();
        limbs_slice_or_neg_neg_in_place_left(&mut xs, ys);
        assert_eq!(xs, xs_after);
    };
    test(&[2], &[3], vec![1]);
    test(&[1, 1, 1], &[1, 2, 3], vec![1, 0, 1]);
    test(&[6, 7], &[1, 2, 3], vec![1, 2]);
    test(&[1, 2, 3], &[6, 7], vec![1, 2, 0]);
    test(&[100, 101, 102], &[102, 101, 100], vec![98, 101, 100]);
    test(&[0, 0, 1], &[3], vec![3, 0, 0]);
    test(&[3], &[0, 0, 1], vec![3]);
    test(&[0, 3, 3], &[0, 0, 3], vec![0, 3, 2]);
    test(&[0, 0, 3], &[0, 3, 3], vec![0, 3, 2]);
    test(&[0, 3], &[0, 0, 3], vec![0, 3]);
    test(&[0, 0, 3], &[0, 3], vec![0, 3, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_slice_or_neg_neg_in_place_left_fail_1() {
    limbs_slice_or_neg_neg_in_place_left(&mut [0, 0, 0], &[3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_slice_or_neg_neg_in_place_left_fail_2() {
    limbs_slice_or_neg_neg_in_place_left(&mut [3], &[0, 0, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_vec_or_neg_neg_in_place_left_fail_1() {
    limbs_vec_or_neg_neg_in_place_left(&mut vec![0, 0, 0], &[3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_vec_or_neg_neg_in_place_left_fail_2() {
    limbs_vec_or_neg_neg_in_place_left(&mut vec![3], &[0, 0, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_or_neg_neg_in_place_either() {
    let test = |xs_before: &[Limb], ys_before: &[Limb], b, xs_after, ys_after| {
        let mut xs = xs_before.to_vec();
        let mut ys = ys_before.to_vec();
        assert_eq!(limbs_or_neg_neg_in_place_either(&mut xs, &mut ys), b);
        assert_eq!(xs, xs_after);
        assert_eq!(ys, ys_after);
    };
    test(&[2], &[3], false, vec![1], vec![3]);
    test(&[1, 1, 1], &[1, 2, 3], false, vec![1, 0, 1], vec![1, 2, 3]);
    test(&[1, 2, 3], &[6, 7], true, vec![1, 2, 3], vec![1, 2]);
    test(&[6, 7], &[1, 2, 3], false, vec![1, 2], vec![1, 2, 3]);
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        false,
        vec![98, 101, 100],
        vec![102, 101, 100],
    );
    test(&[0, 0, 1], &[3], true, vec![0, 0, 1], vec![3]);
    test(&[3], &[0, 0, 1], false, vec![3], vec![0, 0, 1]);
    test(&[0, 3, 3], &[0, 0, 3], false, vec![0, 3, 2], vec![0, 0, 3]);
    test(&[0, 0, 3], &[0, 3, 3], false, vec![0, 3, 2], vec![0, 3, 3]);
    test(&[0, 3], &[0, 0, 3], false, vec![0, 3], vec![0, 0, 3]);
    test(&[0, 0, 3], &[0, 3], true, vec![0, 0, 3], vec![0, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_or_neg_neg_in_place_either_fail_1() {
    limbs_or_neg_neg_in_place_either(&mut [0, 0, 0], &mut [3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_or_neg_neg_in_place_either_fail_2() {
    limbs_or_neg_neg_in_place_either(&mut [3], &mut [0, 0, 0]);
}

#[test]
fn test_or() {
    let test = |s, t, out| {
        let u = Integer::from_str(s).unwrap();
        let v = Integer::from_str(t).unwrap();

        let mut n = u.clone();
        n |= v.clone();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = u.clone();
        n |= &v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone() | v.clone();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &u | v.clone();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone() | &v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &u | &v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        assert_eq!(integer_or_alt_1(&u, &v).to_string(), out);
        assert_eq!(integer_or_alt_2(&u, &v).to_string(), out);

        let n = rug::Integer::from_str(s).unwrap() | rug::Integer::from_str(t).unwrap();
        assert_eq!(n.to_string(), out);
    };
    test("0", "0", "0");
    test("0", "123", "123");
    test("123", "0", "123");
    test("123", "456", "507");
    test("1000000000000", "123", "1000000000123");
    test("123", "1000000000000", "1000000000123");
    test("1000000000000", "999999999999", "1000000004095");
    test("12345678987654321", "314159265358979", "12347506587071667");
    test("0", "-123", "-123");
    test("123", "-456", "-389");
    test("1000000000000", "-123", "-123");
    test("123", "-1000000000000", "-999999999877");
    test("1000000000000", "-999999999999", "-4095");
    test("12345678987654321", "-314159265358979", "-1827599417347");
    test("-123", "0", "-123");
    test("-123", "456", "-51");
    test("-1000000000000", "123", "-999999999877");
    test("-123", "1000000000000", "-123");
    test("-1000000000000", "999999999999", "-1");
    test(
        "-12345678987654321",
        "314159265358979",
        "-12033347321712689",
    );
    test("-123", "-456", "-67");
    test("-1000000000000", "-123", "-123");
    test("-123", "-1000000000000", "-123");
    test("-1000000000000", "-999999999999", "-999999999999");
    test("-12345678987654321", "-314159265358979", "-312331665941633");

    test(
        "17561442137713604341197",
        "-533163900219836",
        "-75045493870643",
    );
    test(
        "-18446744013580009457",
        "-18446673704965373937",
        "-18446673644835831793",
    );
    test(
        "-18446673704965373937",
        "-18446744013580009457",
        "-18446673644835831793",
    );
    test(
        "-324518553658389833295008601473024",
        "317057721155483154675232931839",
        "-324201495937234350140333368541185",
    );
    test(
        "317057721155483154675232931839",
        "-324518553658389833295008601473024",
        "-324201495937234350140333368541185",
    );
    test(
        "-324201495937234350140333368541185",
        "-324518553658389833295008601473024",
        "-324201495937234350140333368541185",
    );
    test(
        "-324518553658389833295008601473024",
        "-324201495937234350140333368541185",
        "-324201495937234350140333368541185",
    );
    test(
        "576458553284361984",
        "-10889035741470030830237691627457877114880",
        "-10889035741470030830237115168904592752896",
    );
    test(
        "-26298808336",
        "170141183460469156173823577801560686592",
        "-26298808336",
    );
    test(
        "-4363947867655",
        "-158453907176889445928738488320",
        "-4363947867655",
    );
}

#[test]
fn limbs_neg_or_limb_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_18().test_properties_with_config(&config, |(xs, y)| {
        assert_eq!(
            -Natural::from_owned_limbs_asc(limbs_neg_or_limb(&xs, y)),
            -Natural::from_owned_limbs_asc(xs) | Integer::from(y)
        );
    });
}

#[test]
fn limbs_neg_or_limb_to_out_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_5().test_properties_with_config(
        &config,
        |(mut out, xs, y)| {
            let old_out = out.clone();
            limbs_neg_or_limb_to_out(&mut out, &xs, y);
            let len = xs.len();
            assert_eq!(
                -Natural::from_limbs_asc(&out[..len]),
                -Natural::from_owned_limbs_asc(xs) | Integer::from(y),
            );
            assert_eq!(&out[len..], &old_out[len..]);
        },
    );
}

#[test]
fn limbs_neg_or_limb_in_place_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_18().test_properties_with_config(&config, |(mut xs, y)| {
        let old_xs = xs.clone();
        limbs_neg_or_limb_in_place(&mut xs, y);
        assert_eq!(
            -Natural::from_owned_limbs_asc(xs),
            -Natural::from_owned_limbs_asc(old_xs) | Integer::from(y)
        );
    });
}

#[test]
fn limbs_pos_or_neg_limb_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_22().test_properties_with_config(&config, |(xs, y)| {
        assert_eq!(
            limbs_pos_or_neg_limb(&xs, y),
            -(Integer::from(Natural::from_owned_limbs_asc(xs))
                | Integer::from_owned_twos_complement_limbs_asc(vec![y, Limb::MAX]))
        );
    });
}

#[test]
fn limbs_neg_or_neg_limb_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_22().test_properties_with_config(&config, |(xs, y)| {
        assert_eq!(
            limbs_neg_or_neg_limb(&xs, y),
            -(-Natural::from_owned_limbs_asc(xs)
                | Integer::from_owned_twos_complement_limbs_asc(vec![y, Limb::MAX]))
        );
    });
}

#[test]
fn limbs_or_pos_neg_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen_var_8().test_properties_with_config(&config, |(xs, ys)| {
        assert_eq!(
            -Natural::from_owned_limbs_asc(limbs_or_pos_neg(&xs, &ys)),
            Integer::from(Natural::from_owned_limbs_asc(xs)) | -Natural::from_owned_limbs_asc(ys)
        );
    });
}

#[test]
fn limbs_or_pos_neg_to_out_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_triple_gen_var_33().test_properties_with_config(&config, |(mut out, ys, xs)| {
        let out_old = out.clone();
        limbs_or_pos_neg_to_out(&mut out, &xs, &ys);
        let len = ys.len();
        assert_eq!(
            -Natural::from_limbs_asc(&out[..len]),
            Integer::from(Natural::from_owned_limbs_asc(xs)) | -Natural::from_owned_limbs_asc(ys)
        );
        assert_eq!(&out[len..], &out_old[len..]);
    });
}

#[test]
fn limbs_slice_or_pos_neg_in_place_left_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen_var_8().test_properties_with_config(&config, |(mut xs, ys)| {
        let xs_old = xs.clone();
        if limbs_slice_or_pos_neg_in_place_left(&mut xs, &ys) {
            let mut out = Natural::exact_from(
                -(Integer::from(Natural::from_owned_limbs_asc(xs_old))
                    | -Natural::from_owned_limbs_asc(ys)),
            )
            .to_limbs_asc();
            out.resize(xs.len(), 0);
            assert_eq!(out, xs);
        } else {
            assert_eq!(
                -Natural::from_owned_limbs_asc(xs),
                Integer::from(Natural::from_owned_limbs_asc(xs_old))
                    | -Natural::from_owned_limbs_asc(ys)
            );
        }
    });
}

#[test]
fn limbs_vec_or_pos_neg_in_place_left_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen_var_8().test_properties_with_config(&config, |(mut xs, ys)| {
        let xs_old = xs.clone();
        limbs_vec_or_pos_neg_in_place_left(&mut xs, &ys);
        assert_eq!(
            -Natural::from_owned_limbs_asc(xs),
            Integer::from(Natural::from_owned_limbs_asc(xs_old))
                | -Natural::from_owned_limbs_asc(ys)
        );
    });
}

#[test]
fn limbs_or_pos_neg_in_place_right_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen_var_8().test_properties_with_config(&config, |(xs, mut ys)| {
        let ys_old = ys.clone();
        limbs_or_pos_neg_in_place_right(&xs, &mut ys);
        assert_eq!(
            -Natural::from_owned_limbs_asc(ys),
            Integer::from(Natural::from_owned_limbs_asc(xs))
                | -Natural::from_owned_limbs_asc(ys_old)
        );
    });
}

#[test]
fn limbs_or_neg_neg_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen_var_8().test_properties_with_config(&config, |(xs, ys)| {
        assert_eq!(
            -Natural::from_owned_limbs_asc(limbs_or_neg_neg(&xs, &ys)),
            -Natural::from_owned_limbs_asc(xs) | -Natural::from_owned_limbs_asc(ys)
        );
    });
}

#[test]
fn limbs_or_neg_neg_to_out_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_triple_gen_var_35().test_properties_with_config(&config, |(mut out, xs, ys)| {
        let out_old = out.clone();
        limbs_or_neg_neg_to_out(&mut out, &xs, &ys);
        let len = min(xs.len(), ys.len());
        let result = Natural::exact_from(
            -(-Natural::from_owned_limbs_asc(xs) | -Natural::from_owned_limbs_asc(ys)),
        );
        let mut expected_limbs = result.into_limbs_asc();
        expected_limbs.resize(len, 0);
        assert_eq!(&out[..len], expected_limbs);
        assert_eq!(&out[len..], &out_old[len..]);
    });
}

#[test]
fn limbs_slice_or_neg_neg_in_place_left_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen_var_8().test_properties_with_config(&config, |(mut xs, ys)| {
        let xs_old = xs.clone();
        limbs_slice_or_neg_neg_in_place_left(&mut xs, &ys);
        assert_eq!(
            -Natural::from_owned_limbs_asc(xs),
            -Natural::from_owned_limbs_asc(xs_old) | -Natural::from_owned_limbs_asc(ys)
        );
    });
}

#[test]
fn limbs_vec_or_neg_neg_in_place_left_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen_var_8().test_properties_with_config(&config, |(mut xs, ys)| {
        let xs_old = xs.clone();
        limbs_vec_or_neg_neg_in_place_left(&mut xs, &ys);
        assert_eq!(
            -Natural::from_owned_limbs_asc(xs),
            -Natural::from_owned_limbs_asc(xs_old) | -Natural::from_owned_limbs_asc(ys)
        );
    });
}

#[test]
fn limbs_or_neg_neg_in_place_either_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen_var_8().test_properties_with_config(&config, |(mut xs, mut ys)| {
        let xs_old = xs.clone();
        let ys_old = ys.clone();
        let right = limbs_or_neg_neg_in_place_either(&mut xs, &mut ys);
        let expected = -Natural::from_limbs_asc(&xs_old) | -Natural::from_limbs_asc(&ys_old);
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
fn or_properties() {
    integer_pair_gen().test_properties(|(x, y)| {
        let result_val_val = x.clone() | y.clone();
        let result_val_ref = x.clone() | &y;
        let result_ref_val = &x | y.clone();
        let result = &x | &y;
        assert!(result_val_val.is_valid());
        assert!(result_val_ref.is_valid());
        assert!(result_ref_val.is_valid());
        assert!(result.is_valid());
        assert_eq!(result_val_val, result);
        assert_eq!(result_val_ref, result);
        assert_eq!(result_ref_val, result);

        let mut mut_x = x.clone();
        mut_x |= y.clone();
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, result);
        let mut mut_x = x.clone();
        mut_x |= &y;
        assert_eq!(mut_x, result);
        assert!(mut_x.is_valid());

        let mut mut_x = rug::Integer::from(&x);
        mut_x |= rug::Integer::from(&y);
        assert_eq!(Integer::from(&mut_x), result);

        assert_eq!(
            Integer::from(&(rug::Integer::from(&x) | rug::Integer::from(&y))),
            result
        );

        assert_eq!(integer_or_alt_1(&x, &y), result);
        assert_eq!(integer_or_alt_2(&x, &y), result);

        assert_eq!(&y | &x, result);
        assert_eq!(&result | &x, result);
        assert_eq!(&result | &y, result);
        assert_eq!(!(!x & !y), result);
    });

    integer_gen().test_properties(|x| {
        assert_eq!(&x | Integer::ZERO, x);
        assert_eq!(Integer::ZERO | &x, x);
        assert_eq!(&x | Integer::NEGATIVE_ONE, -1);
        assert_eq!(Integer::NEGATIVE_ONE | &x, -1);
        assert_eq!(&x | &x, x);
        assert_eq!(&x | !&x, -1);
    });

    integer_triple_gen().test_properties(|(ref x, ref y, ref z)| {
        assert_eq!((x | y) | z, x | (y | z));
        assert_eq!(x & (y | z), (x & y) | (x & z));
        assert_eq!((x & y) | z, (x | z) & (y | z));
        assert_eq!(x | (y & z), (x | y) & (x | z));
        assert_eq!((x | y) & z, (x & z) | (y & z));
    });

    signed_pair_gen::<SignedLimb>().test_properties(|(i, j)| {
        assert_eq!(Integer::from(i) | Integer::from(j), i | j);
    });

    natural_pair_gen().test_properties(|(x, y)| {
        assert_eq!(Integer::from(&x) | Integer::from(&y), x | y);
    });
}
