// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{
    large_type_gen_var_9, unsigned_pair_gen_var_27, unsigned_vec_pair_gen,
    unsigned_vec_pair_gen_var_1, unsigned_vec_pair_gen_var_6, unsigned_vec_triple_gen_var_31,
    unsigned_vec_triple_gen_var_32, unsigned_vec_triple_gen_var_40, unsigned_vec_unsigned_pair_gen,
    unsigned_vec_unsigned_pair_gen_var_15, unsigned_vec_unsigned_vec_bool_triple_gen_var_1,
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_1,
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_11,
};
use malachite_base::vecs::vec_from_str;
use malachite_nz::natural::arithmetic::add::{
    limbs_add, limbs_add_greater, limbs_add_greater_to_out, limbs_add_limb, limbs_add_limb_to_out,
    limbs_add_same_length_to_out, limbs_add_same_length_with_carry_in_in_place_left,
    limbs_add_same_length_with_carry_in_to_out, limbs_add_to_out, limbs_add_to_out_aliased,
    limbs_slice_add_greater_in_place_left, limbs_slice_add_in_place_either,
    limbs_slice_add_limb_in_place, limbs_slice_add_same_length_in_place_left,
    limbs_vec_add_in_place_either, limbs_vec_add_in_place_left, limbs_vec_add_limb_in_place,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::{DoubleLimb, Limb};
use malachite_nz::test_util::generators::{
    natural_gen, natural_pair_gen, natural_triple_gen, natural_vec_gen,
};
use malachite_nz::test_util::natural::arithmetic::add::natural_sum_alt;
use num::BigUint;
use rug;
use std::cmp::max;
use std::iter::{once, Sum};
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_add_limb() {
    let test = |xs: &[Limb], y: Limb, out: &[Limb]| {
        assert_eq!(limbs_add_limb(xs, y), out);
    };
    test(&[], 0, &[]);
    test(&[], 5, &[5]);
    test(&[6, 7], 2, &[8, 7]);
    test(&[100, 101, 102], 10, &[110, 101, 102]);
    test(&[123, 456], 789, &[912, 456]);
    test(&[u32::MAX, 5], 2, &[1, 6]);
    test(&[u32::MAX], 2, &[1, 1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_add_limb_to_out() {
    let test = |out_before: &[Limb], xs: &[Limb], y: Limb, carry: bool, out_after: &[Limb]| {
        let mut out = out_before.to_vec();
        assert_eq!(limbs_add_limb_to_out(&mut out, xs, y), carry);
        assert_eq!(out, out_after);
    };
    test(&[10, 10, 10, 10], &[], 0, false, &[10, 10, 10, 10]);
    test(&[10, 10, 10, 10], &[], 5, true, &[10, 10, 10, 10]);
    test(&[10, 10, 10, 10], &[6, 7], 2, false, &[8, 7, 10, 10]);
    test(
        &[10, 10, 10, 10],
        &[100, 101, 102],
        10,
        false,
        &[110, 101, 102, 10],
    );
    test(
        &[10, 10, 10, 10],
        &[123, 456],
        789,
        false,
        &[912, 456, 10, 10],
    );
    test(&[10, 10, 10, 10], &[u32::MAX, 5], 2, false, &[1, 6, 10, 10]);
    test(&[10, 10, 10, 10], &[u32::MAX], 2, true, &[1, 10, 10, 10]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_add_limb_to_out_fail() {
    limbs_add_limb_to_out(&mut [10], &[10, 10], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_slice_add_limb_in_place() {
    let test = |xs: &[Limb], y: Limb, carry: bool, out: &[Limb]| {
        let mut xs = xs.to_vec();
        assert_eq!(limbs_slice_add_limb_in_place(&mut xs, y), carry);
        assert_eq!(xs, out);
    };
    test(&[], 0, false, &[]);
    test(&[], 5, true, &[]);
    test(&[6, 7], 2, false, &[8, 7]);
    test(&[100, 101, 102], 10, false, &[110, 101, 102]);
    test(&[123, 456], 789, false, &[912, 456]);
    test(&[u32::MAX, 5], 2, false, &[1, 6]);
    test(&[u32::MAX], 2, true, &[1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_vec_add_limb_in_place() {
    let test = |xs: &[Limb], y: Limb, out: &[Limb]| {
        let mut xs = xs.to_vec();
        limbs_vec_add_limb_in_place(&mut xs, y);
        assert_eq!(xs, out);
    };
    test(&[6, 7], 2, &[8, 7]);
    test(&[100, 101, 102], 10, &[110, 101, 102]);
    test(&[123, 456], 789, &[912, 456]);
    test(&[u32::MAX, 5], 2, &[1, 6]);
    test(&[u32::MAX], 2, &[1, 1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_vec_add_limb_in_place_fail() {
    limbs_vec_add_limb_in_place(&mut vec![], 10);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_add_greater() {
    let test = |xs, ys, out| {
        assert_eq!(limbs_add_greater(xs, ys), out);
    };
    test(&[], &[], vec![]);
    test(&[2], &[], vec![2]);
    test(&[2], &[3], vec![5]);
    test(&[1, 1, 1], &[1, 2, 3], vec![2, 3, 4]);
    test(&[1, 2, 3], &[6, 7], vec![7, 9, 3]);
    test(&[100, 101, 102], &[102, 101, 100], vec![202, 202, 202]);
    test(&[u32::MAX, 3], &[1], vec![0, 4]);
    test(&[u32::MAX], &[1], vec![0, 1]);
    test(&[1], &[u32::MAX], vec![0, 1]);
    test(&[u32::MAX], &[u32::MAX], vec![u32::MAX - 1, 1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn test_limbs_add_greater_fail() {
    limbs_add_greater(&[6, 7], &[1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_add_and_limbs_vec_add_in_place_left() {
    let test = |xs_before, ys, out| {
        assert_eq!(limbs_add(xs_before, ys), out);

        let mut xs = xs_before.to_vec();
        limbs_vec_add_in_place_left(&mut xs, ys);
        assert_eq!(xs, out);
    };
    test(&[], &[], vec![]);
    test(&[2], &[], vec![2]);
    test(&[], &[2], vec![2]);
    test(&[2], &[3], vec![5]);
    test(&[1, 1, 1], &[1, 2, 3], vec![2, 3, 4]);
    test(&[6, 7], &[1, 2, 3], vec![7, 9, 3]);
    test(&[1, 2, 3], &[6, 7], vec![7, 9, 3]);
    test(&[100, 101, 102], &[102, 101, 100], vec![202, 202, 202]);
    test(&[u32::MAX, 3], &[1], vec![0, 4]);
    test(&[1], &[u32::MAX, 3], vec![0, 4]);
    test(&[u32::MAX], &[1], vec![0, 1]);
    test(&[1], &[u32::MAX], vec![0, 1]);
    test(&[u32::MAX], &[u32::MAX], vec![u32::MAX - 1, 1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_add_greater_to_out() {
    let test = |xs, ys, out_before: &[Limb], carry, out_after| {
        let mut out = out_before.to_vec();
        assert_eq!(limbs_add_greater_to_out(&mut out, xs, ys), carry);
        assert_eq!(out, out_after);
    };
    test(&[], &[], &[0, 0], false, vec![0, 0]);
    test(&[2], &[], &[0, 0], false, vec![2, 0]);
    test(&[2], &[3], &[0, 0], false, vec![5, 0]);
    test(
        &[1, 1, 1],
        &[1, 2, 3],
        &[5, 5, 5, 5],
        false,
        vec![2, 3, 4, 5],
    );
    test(&[1, 2, 3], &[6, 7], &[0, 0, 0], false, vec![7, 9, 3]);
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        &[10, 10, 10, 10],
        false,
        vec![202, 202, 202, 10],
    );
    test(
        &[u32::MAX, 3],
        &[1],
        &[10, 10, 10, 10],
        false,
        vec![0, 4, 10, 10],
    );
    test(
        &[u32::MAX],
        &[1],
        &[10, 10, 10, 10],
        true,
        vec![0, 10, 10, 10],
    );
    test(
        &[1],
        &[u32::MAX],
        &[10, 10, 10, 10],
        true,
        vec![0, 10, 10, 10],
    );
    test(
        &[u32::MAX],
        &[u32::MAX],
        &[10, 10, 10, 10],
        true,
        vec![u32::MAX - 1, 10, 10, 10],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_add_greater_to_out_fail_1() {
    let mut out = vec![10, 10, 10, 10];
    limbs_add_greater_to_out(&mut out, &[6, 7], &[1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_add_greater_to_out_fail_2() {
    let mut out = vec![10];
    limbs_add_greater_to_out(&mut out, &[6, 7], &[1, 2]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_add_same_length_to_out() {
    let test = |xs, ys, out_before: &[Limb], carry, out_after| {
        let mut out = out_before.to_vec();
        assert_eq!(limbs_add_same_length_to_out(&mut out, xs, ys), carry);
        assert_eq!(out, out_after);
    };
    test(&[], &[], &[0, 0], false, vec![0, 0]);
    test(&[2], &[3], &[0, 0], false, vec![5, 0]);
    test(
        &[1, 1, 1],
        &[1, 2, 3],
        &[5, 5, 5, 5],
        false,
        vec![2, 3, 4, 5],
    );
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        &[10, 10, 10, 10],
        false,
        vec![202, 202, 202, 10],
    );
    test(
        &[u32::MAX],
        &[1],
        &[10, 10, 10, 10],
        true,
        vec![0, 10, 10, 10],
    );
    test(
        &[1],
        &[u32::MAX],
        &[10, 10, 10, 10],
        true,
        vec![0, 10, 10, 10],
    );
    test(
        &[u32::MAX],
        &[u32::MAX],
        &[10, 10, 10, 10],
        true,
        vec![u32::MAX - 1, 10, 10, 10],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_add_same_length_to_out_fail_1() {
    let mut out = vec![10, 10, 10, 10];
    limbs_add_same_length_to_out(&mut out, &[6, 7], &[1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_add_same_length_to_out_fail_2() {
    let mut out = vec![10];
    limbs_add_same_length_to_out(&mut out, &[6, 7], &[1, 2]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_add_to_out() {
    let test = |xs, ys, out_before: &[Limb], carry, out_after| {
        let mut out = out_before.to_vec();
        assert_eq!(limbs_add_to_out(&mut out, xs, ys), carry);
        assert_eq!(out, out_after);
    };
    test(&[], &[], &[0, 0], false, vec![0, 0]);
    test(&[2], &[], &[0, 0], false, vec![2, 0]);
    test(&[], &[2], &[0, 0], false, vec![2, 0]);
    test(&[2], &[3], &[0, 0], false, vec![5, 0]);
    test(
        &[1, 1, 1],
        &[1, 2, 3],
        &[5, 5, 5, 5],
        false,
        vec![2, 3, 4, 5],
    );
    test(&[6, 7], &[1, 2, 3], &[0, 0, 0], false, vec![7, 9, 3]);
    test(&[1, 2, 3], &[6, 7], &[0, 0, 0], false, vec![7, 9, 3]);
    test(
        &[6, 7],
        &[1, 2, 3],
        &[10, 10, 10, 10],
        false,
        vec![7, 9, 3, 10],
    );
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        &[10, 10, 10, 10],
        false,
        vec![202, 202, 202, 10],
    );
    test(
        &[u32::MAX, 3],
        &[1],
        &[10, 10, 10, 10],
        false,
        vec![0, 4, 10, 10],
    );
    test(
        &[1],
        &[u32::MAX, 3],
        &[10, 10, 10, 10],
        false,
        vec![0, 4, 10, 10],
    );
    test(
        &[u32::MAX],
        &[1],
        &[10, 10, 10, 10],
        true,
        vec![0, 10, 10, 10],
    );
    test(
        &[1],
        &[u32::MAX],
        &[10, 10, 10, 10],
        true,
        vec![0, 10, 10, 10],
    );
    test(
        &[u32::MAX],
        &[u32::MAX],
        &[10, 10, 10, 10],
        true,
        vec![u32::MAX - 1, 10, 10, 10],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_add_to_out_fail() {
    let mut out = vec![10, 10];
    limbs_add_to_out(&mut out, &[6, 7, 8], &[1, 2]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_add_to_out_aliased() {
    let test = |xs_before: &[Limb], in_size, ys: &[Limb], carry, xs_after| {
        let mut xs = xs_before.to_vec();
        assert_eq!(limbs_add_to_out_aliased(&mut xs, in_size, ys), carry);
        assert_eq!(xs, xs_after);
    };
    test(&[], 0, &[], false, vec![]);
    test(&[1, 2, 3], 0, &[4, 5], false, vec![4, 5, 3]);
    test(&[1, 2, 3], 1, &[4, 5], false, vec![5, 5, 3]);
    test(&[1, 2, 3], 2, &[4, 5], false, vec![5, 7, 3]);
    test(&[1, 1, 3], 1, &[u32::MAX, 5], false, vec![0, 6, 3]);
    test(&[1, 1, 3], 1, &[u32::MAX], true, vec![0, 1, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_add_to_out_aliased_fail_1() {
    let mut out = vec![6, 7];
    limbs_add_to_out_aliased(&mut out, 1, &[1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_add_to_out_aliased_fail_2() {
    let mut out = vec![6, 7, 8, 9];
    limbs_add_to_out_aliased(&mut out, 4, &[1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_slice_add_same_length_in_place_left() {
    let test = |xs_before: &[Limb], ys, carry, xs_after| {
        let mut xs = xs_before.to_vec();
        assert_eq!(
            limbs_slice_add_same_length_in_place_left(&mut xs, ys),
            carry
        );
        assert_eq!(xs, xs_after);
    };
    test(&[], &[], false, vec![]);
    test(&[2], &[3], false, vec![5]);
    test(&[1, 1, 1], &[1, 2, 3], false, vec![2, 3, 4]);
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        false,
        vec![202, 202, 202],
    );
    test(&[u32::MAX], &[1], true, vec![0]);
    test(&[1], &[u32::MAX], true, vec![0]);
    test(&[u32::MAX], &[u32::MAX], true, vec![u32::MAX - 1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_slice_add_same_length_in_place_left_fail() {
    let mut out = vec![6, 7];
    limbs_slice_add_same_length_in_place_left(&mut out, &[1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_slice_add_greater_in_place_left() {
    let test = |xs_before: &[Limb], ys, carry, xs_after| {
        let mut xs = xs_before.to_vec();
        assert_eq!(limbs_slice_add_greater_in_place_left(&mut xs, ys), carry);
        assert_eq!(xs, xs_after);
    };
    test(&[], &[], false, vec![]);
    test(&[2], &[], false, vec![2]);
    test(&[2], &[3], false, vec![5]);
    test(&[1, 1, 1], &[1, 2, 3], false, vec![2, 3, 4]);
    test(&[1, 2, 3], &[6, 7], false, vec![7, 9, 3]);
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        false,
        vec![202, 202, 202],
    );
    test(&[u32::MAX, 3], &[1], false, vec![0, 4]);
    test(&[u32::MAX], &[1], true, vec![0]);
    test(&[1], &[u32::MAX], true, vec![0]);
    test(&[u32::MAX], &[u32::MAX], true, vec![u32::MAX - 1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_slice_add_greater_in_place_left_fail() {
    let mut out = vec![6, 7];
    limbs_slice_add_greater_in_place_left(&mut out, &[1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_slice_add_in_place_either() {
    let test = |xs_before: &[Limb], ys_before: &[Limb], right, xs_after, ys_after| {
        let mut xs = xs_before.to_vec();
        let mut ys = ys_before.to_vec();
        assert_eq!(limbs_slice_add_in_place_either(&mut xs, &mut ys), right);
        assert_eq!(xs, xs_after);
        assert_eq!(ys, ys_after);
    };
    test(&[], &[], (false, false), vec![], vec![]);
    test(&[2], &[], (false, false), vec![2], vec![]);
    test(&[], &[2], (true, false), vec![], vec![2]);
    test(&[2], &[3], (false, false), vec![5], vec![3]);
    test(&[6, 7], &[1, 2], (false, false), vec![7, 9], vec![1, 2]);
    test(
        &[6, 7],
        &[1, 2, 3],
        (true, false),
        vec![6, 7],
        vec![7, 9, 3],
    );
    test(
        &[1, 2, 3],
        &[6, 7],
        (false, false),
        vec![7, 9, 3],
        vec![6, 7],
    );
    test(&[], &[1, 2, 3], (true, false), vec![], vec![1, 2, 3]);
    test(&[1, 2, 3], &[], (false, false), vec![1, 2, 3], vec![]);
    test(
        &[1, 1, 1],
        &[1, 2, 3],
        (false, false),
        vec![2, 3, 4],
        vec![1, 2, 3],
    );
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        (false, false),
        vec![202, 202, 202],
        vec![102, 101, 100],
    );
    test(&[u32::MAX, 3], &[1], (false, false), vec![0, 4], vec![1]);
    test(&[1], &[u32::MAX, 3], (true, false), vec![1], vec![0, 4]);
    test(&[u32::MAX], &[1], (false, true), vec![0], vec![1]);
    test(&[1], &[u32::MAX], (false, true), vec![0], vec![u32::MAX]);
    test(
        &[u32::MAX],
        &[u32::MAX],
        (false, true),
        vec![u32::MAX - 1],
        vec![u32::MAX],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_vec_add_in_place_either() {
    let test = |xs_before: &[Limb], ys_before: &[Limb], right, xs_after, ys_after| {
        let mut xs = xs_before.to_vec();
        let mut ys = ys_before.to_vec();
        assert_eq!(limbs_vec_add_in_place_either(&mut xs, &mut ys), right);
        assert_eq!(xs, xs_after);
        assert_eq!(ys, ys_after);
    };
    test(&[], &[], false, vec![], vec![]);
    test(&[2], &[], false, vec![2], vec![]);
    test(&[], &[2], true, vec![], vec![2]);
    test(&[2], &[3], false, vec![5], vec![3]);
    test(&[6, 7], &[1, 2], false, vec![7, 9], vec![1, 2]);
    test(&[6, 7], &[1, 2, 3], true, vec![6, 7], vec![7, 9, 3]);
    test(&[1, 2, 3], &[6, 7], false, vec![7, 9, 3], vec![6, 7]);
    test(&[], &[1, 2, 3], true, vec![], vec![1, 2, 3]);
    test(&[1, 2, 3], &[], false, vec![1, 2, 3], vec![]);
    test(&[1, 1, 1], &[1, 2, 3], false, vec![2, 3, 4], vec![1, 2, 3]);
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        false,
        vec![202, 202, 202],
        vec![102, 101, 100],
    );
    test(&[u32::MAX, 3], &[1], false, vec![0, 4], vec![1]);
    test(&[1], &[u32::MAX, 3], true, vec![1], vec![0, 4]);
    test(&[u32::MAX], &[1], false, vec![0, 1], vec![1]);
    test(&[1], &[u32::MAX], false, vec![0, 1], vec![u32::MAX]);
    test(
        &[u32::MAX],
        &[u32::MAX],
        false,
        vec![u32::MAX - 1, 1],
        vec![u32::MAX],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_add_same_length_with_carry_in_to_out() {
    let test = |xs, ys, carry_in, out_before: &[Limb], carry, out_after| {
        let mut out = out_before.to_vec();
        assert_eq!(
            limbs_add_same_length_with_carry_in_to_out(&mut out, xs, ys, carry_in),
            carry
        );
        assert_eq!(out, out_after);
    };
    test(&[], &[], false, &[0, 0], false, vec![0, 0]);
    test(&[], &[], true, &[0, 0], true, vec![0, 0]);
    test(&[2], &[3], false, &[0, 0], false, vec![5, 0]);
    test(&[2], &[3], true, &[0, 0], false, vec![6, 0]);
    test(
        &[1, 1, 1],
        &[1, 2, 3],
        false,
        &[5, 5, 5, 5],
        false,
        vec![2, 3, 4, 5],
    );
    test(
        &[1, 1, 1],
        &[1, 2, 3],
        true,
        &[5, 5, 5, 5],
        false,
        vec![3, 3, 4, 5],
    );
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        false,
        &[10, 10, 10, 10],
        false,
        vec![202, 202, 202, 10],
    );
    test(
        &[u32::MAX],
        &[1],
        false,
        &[10, 10, 10, 10],
        true,
        vec![0, 10, 10, 10],
    );
    test(
        &[u32::MAX],
        &[1],
        true,
        &[10, 10, 10, 10],
        true,
        vec![1, 10, 10, 10],
    );
    test(
        &[u32::MAX - 1],
        &[1],
        true,
        &[10, 10, 10, 10],
        true,
        vec![0, 10, 10, 10],
    );
    test(
        &[1],
        &[u32::MAX],
        false,
        &[10, 10, 10, 10],
        true,
        vec![0, 10, 10, 10],
    );
    test(
        &[u32::MAX],
        &[u32::MAX],
        false,
        &[10, 10, 10, 10],
        true,
        vec![u32::MAX - 1, 10, 10, 10],
    );
    test(
        &[u32::MAX],
        &[u32::MAX],
        true,
        &[10, 10, 10, 10],
        true,
        vec![u32::MAX, 10, 10, 10],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_add_same_length_with_carry_in_to_out_fail_1() {
    let mut out = vec![10, 10, 10, 10];
    limbs_add_same_length_with_carry_in_to_out(&mut out, &[6, 7], &[1, 2, 3], false);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_add_same_length_with_carry_in_to_out_fail_2() {
    let mut out = vec![10];
    limbs_add_same_length_with_carry_in_to_out(&mut out, &[6, 7], &[1, 2], false);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_add_same_length_with_carry_in_in_place_left() {
    let test = |xs_before: &[Limb], ys, carry_in, carry, xs_after| {
        let mut xs = xs_before.to_vec();
        assert_eq!(
            limbs_add_same_length_with_carry_in_in_place_left(&mut xs, ys, carry_in),
            carry
        );
        assert_eq!(xs, xs_after);
    };
    test(&[], &[], false, false, vec![]);
    test(&[], &[], true, true, vec![]);
    test(&[2], &[3], false, false, vec![5]);
    test(&[2], &[3], true, false, vec![6]);
    test(&[1, 1, 1], &[1, 2, 3], false, false, vec![2, 3, 4]);
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        false,
        false,
        vec![202, 202, 202],
    );
    test(&[u32::MAX], &[1], false, true, vec![0]);
    test(&[u32::MAX], &[1], true, true, vec![1]);
    test(&[u32::MAX - 1], &[1], true, true, vec![0]);
    test(&[1], &[u32::MAX], false, true, vec![0]);
    test(&[u32::MAX], &[u32::MAX], false, true, vec![u32::MAX - 1]);
    test(&[u32::MAX], &[u32::MAX], true, true, vec![u32::MAX]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_add_same_length_with_carry_in_in_place_left_fail() {
    let mut out = vec![6, 7];
    limbs_add_same_length_with_carry_in_in_place_left(&mut out, &[1, 2, 3], false);
}

#[test]
fn test_add() {
    let test = |s, t, out| {
        let u = Natural::from_str(s).unwrap();
        let v = Natural::from_str(t).unwrap();

        let mut n = u.clone();
        n += v.clone();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = u.clone();
        n += &v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone() + v.clone();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &u + v.clone();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone() + &v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &u + &v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = BigUint::from_str(s).unwrap() + BigUint::from_str(t).unwrap();
        assert_eq!(n.to_string(), out);

        let n = rug::Integer::from_str(s).unwrap() + rug::Integer::from_str(t).unwrap();
        assert_eq!(n.to_string(), out);
    };
    test("0", "0", "0");
    test("0", "123", "123");
    test("123", "0", "123");
    test("123", "456", "579");
    test("1000000000000", "123", "1000000000123");
    test("123", "1000000000000", "1000000000123");
    test("12345678987654321", "314159265358979", "12659838253013300");
    test(
        "1000000000000",
        "1000000000000000000000000",
        "1000000000001000000000000",
    );
    test(
        "157489031134308824401228232926",
        "2350262829889206551114184866",
        "159839293964198030952342417792",
    );
}

#[test]
fn test_sum() {
    let test = |xs, out| {
        let xs = vec_from_str(xs).unwrap();
        let sum = Natural::sum(xs.iter().cloned());
        assert!(sum.is_valid());
        assert_eq!(sum.to_string(), out);

        let sum_alt = Natural::sum(xs.iter());
        assert!(sum_alt.is_valid());
        assert_eq!(sum_alt, sum);

        let sum_alt = natural_sum_alt(xs.into_iter());
        assert!(sum_alt.is_valid());
        assert_eq!(sum_alt, sum);
    };
    test("[]", "0");
    test("[10]", "10");
    test("[6, 2]", "8");
    test("[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]", "55");
    test("[123456, 789012, 345678, 9012345]", "10270491");
}

#[test]
fn limbs_add_limb_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen().test_properties_with_config(&config, |(xs, y)| {
        assert_eq!(
            Natural::from_owned_limbs_asc(limbs_add_limb(&xs, y)),
            Natural::from_owned_limbs_asc(xs) + Natural::from(y)
        );
    });
}

#[test]
fn limbs_add_limb_to_out_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_1().test_properties_with_config(
        &config,
        |(mut out, xs, y)| {
            let old_out = out.clone();
            let carry = limbs_add_limb_to_out(&mut out, &xs, y);
            let len = xs.len();
            let n = Natural::from_owned_limbs_asc(xs) + Natural::from(y);
            let mut xs = n.into_limbs_asc();
            assert_eq!(carry, xs.len() == len + 1);
            xs.resize(len, 0);
            assert_eq!(xs, &out[..len]);
            assert_eq!(&out[len..], &old_out[len..]);
        },
    );
}

#[test]
fn limbs_slice_add_limb_in_place_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen().test_properties_with_config(&config, |(mut xs, y)| {
        let n = Natural::from_limbs_asc(&xs) + Natural::from(y);
        let carry = limbs_slice_add_limb_in_place(&mut xs, y);
        let mut expected_xs = n.into_limbs_asc();
        assert_eq!(carry, expected_xs.len() == xs.len() + 1);
        expected_xs.resize(xs.len(), 0);
        assert_eq!(xs, expected_xs);
    });
}

#[test]
fn limbs_vec_add_limb_in_place_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_pair_gen_var_15().test_properties_with_config(&config, |(mut xs, y)| {
        let n = Natural::from_limbs_asc(&xs) + Natural::from(y);
        limbs_vec_add_limb_in_place(&mut xs, y);
        assert_eq!(Natural::from_owned_limbs_asc(xs), n);
    });
}

fn limbs_add_helper(f: &dyn Fn(&[Limb], &[Limb]) -> Vec<Limb>, xs: Vec<Limb>, ys: Vec<Limb>) {
    assert_eq!(
        Natural::from_owned_limbs_asc(f(&xs, &ys)),
        Natural::from_owned_limbs_asc(xs) + Natural::from_owned_limbs_asc(ys)
    );
}

#[test]
fn limbs_add_greater_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen_var_1().test_properties_with_config(&config, |(xs, ys)| {
        limbs_add_helper(&limbs_add_greater, xs, ys);
    });
}

#[test]
fn limbs_add_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen().test_properties_with_config(&config, |(xs, ys)| {
        limbs_add_helper(&limbs_add, xs, ys);
    });
}

fn limbs_add_to_out_helper(
    f: &mut dyn FnMut(&mut [Limb], &[Limb], &[Limb]) -> bool,
    out_len: &dyn Fn(usize, usize) -> usize,
    mut out: Vec<Limb>,
    xs: Vec<Limb>,
    ys: Vec<Limb>,
) {
    let old_out = out.clone();
    let carry = f(&mut out, &xs, &ys);
    let len = out_len(xs.len(), ys.len());
    let n = Natural::from_owned_limbs_asc(xs) + Natural::from_owned_limbs_asc(ys);
    let mut xs = n.into_limbs_asc();
    assert_eq!(carry, xs.len() == len + 1);
    xs.resize(len, 0);
    assert_eq!(xs, &out[..len]);
    assert_eq!(&out[len..], &old_out[len..]);
}

#[test]
fn limbs_add_greater_to_out_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_triple_gen_var_40().test_properties_with_config(&config, |(out, xs, ys)| {
        limbs_add_to_out_helper(
            &mut limbs_add_greater_to_out,
            &|xs_len, _| xs_len,
            out,
            xs,
            ys,
        );
    });
}

#[test]
fn limbs_add_same_length_to_out_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_triple_gen_var_31().test_properties_with_config(&config, |(out, xs, ys)| {
        limbs_add_to_out_helper(
            &mut limbs_add_same_length_to_out,
            &|xs_len, _| xs_len,
            out,
            xs,
            ys,
        );
    });
}

#[test]
fn limbs_add_to_out_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_triple_gen_var_32().test_properties_with_config(&config, |(out, xs, ys)| {
        limbs_add_to_out_helper(&mut limbs_add_to_out, &max, out, xs, ys);
    });
}

#[test]
fn limbs_add_to_out_aliased_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_vec_unsigned_triple_gen_var_11().test_properties_with_config(
        &config,
        |(mut xs, ys, xs_len)| {
            let xs_old = xs.clone();
            let carry = limbs_add_to_out_aliased(&mut xs, xs_len, &ys);
            let ys_len = ys.len();
            let n = Natural::from_limbs_asc(&xs_old[..xs_len]) + Natural::from_owned_limbs_asc(ys);
            let mut ns = n.into_limbs_asc();
            if ns.len() < ys_len {
                ns.resize(ys_len, 0);
            }
            assert_eq!(
                Natural::from_limbs_asc(&xs[..ys_len]),
                Natural::from_limbs_asc(&ns[..ys_len]),
            );
            if carry {
                assert_eq!(ns.len(), ys_len + 1);
                assert_eq!(*ns.last().unwrap(), 1);
            } else {
                assert_eq!(ns.len(), ys_len);
            }
            assert_eq!(&xs[ys_len..], &xs_old[ys_len..]);
        },
    );
}

fn limbs_slice_add_in_place_left_helper(
    f: &mut dyn FnMut(&mut [Limb], &[Limb]) -> bool,
    xs: &mut [Limb],
    ys: &[Limb],
) {
    let n = Natural::from_limbs_asc(xs) + Natural::from_limbs_asc(ys);
    let carry = f(xs, ys);
    let len = xs.len();
    let mut ns = n.into_limbs_asc();
    assert_eq!(carry, ns.len() == len + 1);
    ns.resize(len, 0);
    assert_eq!(ns, xs);
}

#[test]
fn limbs_slice_add_same_length_in_place_left_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen_var_6().test_properties_with_config(&config, |(mut xs, ys)| {
        limbs_slice_add_in_place_left_helper(
            &mut limbs_slice_add_same_length_in_place_left,
            &mut xs,
            &ys,
        );
    });
}

#[test]
fn limbs_slice_add_greater_in_place_left_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen_var_1().test_properties_with_config(&config, |(mut xs, ys)| {
        limbs_slice_add_in_place_left_helper(
            &mut limbs_slice_add_greater_in_place_left,
            &mut xs,
            &ys,
        );
    });
}

#[test]
fn limbs_vec_add_in_place_left_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen().test_properties_with_config(&config, |(mut xs, ys)| {
        let xs_old = xs.clone();
        limbs_vec_add_in_place_left(&mut xs, &ys);
        assert_eq!(
            Natural::from_owned_limbs_asc(xs),
            Natural::from_owned_limbs_asc(xs_old) + Natural::from_owned_limbs_asc(ys)
        );
    });
}

#[test]
fn limbs_slice_add_in_place_either_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen().test_properties_with_config(&config, |(mut xs, mut ys)| {
        let xs_old = xs.clone();
        let ys_old = ys.clone();
        let (right, b) = limbs_slice_add_in_place_either(&mut xs, &mut ys);
        let len = max(xs_old.len(), ys_old.len());
        let result = Natural::from_limbs_asc(&xs_old) + Natural::from_limbs_asc(&ys_old);
        let mut expected_out = result.to_limbs_asc();
        expected_out.resize(len, 0);
        assert_eq!(!b, Natural::from_limbs_asc(&expected_out) == result);
        if right {
            assert_eq!(ys, expected_out.as_slice());
            assert_eq!(xs, xs_old);
        } else {
            assert_eq!(xs, expected_out.as_slice());
            assert_eq!(ys, ys_old);
        }
    });
}

#[test]
fn limbs_vec_add_in_place_either_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_pair_gen().test_properties_with_config(&config, |(mut xs, mut ys)| {
        let xs_old = xs.clone();
        let ys_old = ys.clone();
        let right = limbs_vec_add_in_place_either(&mut xs, &mut ys);
        let n = Natural::from_limbs_asc(&xs_old) + Natural::from_limbs_asc(&ys_old);
        if right {
            assert_eq!(xs, xs_old);
            assert_eq!(Natural::from_owned_limbs_asc(ys), n);
        } else {
            assert_eq!(Natural::from_owned_limbs_asc(xs), n);
            assert_eq!(ys, ys_old);
        }
    });
}

#[test]
fn limbs_add_same_length_with_carry_in_to_out_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    large_type_gen_var_9().test_properties_with_config(&config, |(out, xs, ys, carry_in)| {
        let mut out = out.to_vec();
        let old_out = out.clone();
        let carry = limbs_add_same_length_with_carry_in_to_out(&mut out, &xs, &ys, carry_in);
        let len = xs.len();
        let mut n = Natural::from_owned_limbs_asc(xs) + Natural::from_owned_limbs_asc(ys);
        if carry_in {
            n += Natural::ONE;
        }
        let mut ns = n.into_limbs_asc();
        assert_eq!(carry, ns.len() == len + 1);
        ns.resize(len, 0);
        assert_eq!(ns, &out[..len]);
        assert_eq!(&out[len..], &old_out[len..]);
    });
}

#[test]
fn limbs_add_same_length_with_carry_in_in_place_left_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    unsigned_vec_unsigned_vec_bool_triple_gen_var_1().test_properties_with_config(
        &config,
        |(mut xs, ys, carry_in)| {
            let xs_old = xs.clone();
            let carry = limbs_add_same_length_with_carry_in_in_place_left(&mut xs, &ys, carry_in);
            let mut n = Natural::from_owned_limbs_asc(xs_old) + Natural::from_owned_limbs_asc(ys);
            if carry_in {
                n += Natural::ONE;
            }
            let len = xs.len();
            let mut ns = n.into_limbs_asc();
            assert_eq!(carry, ns.len() == len + 1);
            ns.resize(len, 0);
            assert_eq!(ns, xs);
        },
    );
}

#[test]
fn add_properties() {
    natural_pair_gen().test_properties(|(x, y)| {
        let sum_val_val = x.clone() + y.clone();
        let sum_val_ref = x.clone() + &y;
        let sum_ref_val = &x + y.clone();
        let sum = &x + &y;
        assert!(sum_val_val.is_valid());
        assert!(sum_val_ref.is_valid());
        assert!(sum_ref_val.is_valid());
        assert!(sum.is_valid());
        assert_eq!(sum_val_val, sum);
        assert_eq!(sum_val_ref, sum);
        assert_eq!(sum_ref_val, sum);

        let mut mut_x = x.clone();
        mut_x += y.clone();
        assert!(mut_x.is_valid());
        assert_eq!(mut_x, sum);
        let mut mut_x = x.clone();
        mut_x += &y;
        assert_eq!(mut_x, sum);
        assert!(mut_x.is_valid());

        let mut mut_x = rug::Integer::from(&x);
        mut_x += rug::Integer::from(&y);
        assert_eq!(Natural::exact_from(&mut_x), sum);

        assert_eq!(Natural::from(&(BigUint::from(&x) + BigUint::from(&y))), sum);
        assert_eq!(
            Natural::exact_from(&(rug::Integer::from(&x) + rug::Integer::from(&y))),
            sum
        );
        assert_eq!(&y + &x, sum);
        assert_eq!(&sum - &x, y);
        assert_eq!(&sum - &y, x);
        assert!(sum >= x);
        assert!(sum >= y);
    });

    natural_triple_gen().test_properties(|(x, y, z)| {
        assert_eq!((&x + &y) + &z, x + (y + z));
    });

    natural_gen().test_properties(|x| {
        assert_eq!(&x + Natural::ZERO, x);
        assert_eq!(Natural::ZERO + &x, x);
        assert_eq!(&x + &x, x << 1);
    });

    unsigned_pair_gen_var_27::<Limb>().test_properties(|(x, y)| {
        assert_eq!(
            DoubleLimb::from(x) + DoubleLimb::from(y),
            Natural::from(x) + Natural::from(y)
        );
    });
}

#[test]
fn sum_properties() {
    natural_vec_gen().test_properties(|xs| {
        let sum = Natural::sum(xs.iter().cloned());
        assert!(sum.is_valid());

        let sum_alt = Natural::sum(xs.iter());
        assert!(sum_alt.is_valid());
        assert_eq!(sum_alt, sum);

        let sum_alt = natural_sum_alt(xs.into_iter());
        assert!(sum_alt.is_valid());
        assert_eq!(sum_alt, sum);
    });

    natural_gen().test_properties(|x| {
        assert_eq!(Natural::sum(once(&x)), x);
        assert_eq!(Natural::sum(once(x.clone())), x);
    });

    natural_pair_gen().test_properties(|(x, y)| {
        let sum = &x + &y;
        assert_eq!(Natural::sum([&x, &y].into_iter()), sum);
        assert_eq!(Natural::sum([x, y].into_iter()), sum);
    });
}
