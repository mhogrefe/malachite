use std::cmp::{max, min};
use std::str::FromStr;

use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
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
use rug;

use malachite_test::common::test_properties;
use malachite_test::common::{integer_to_rug_integer, rug_integer_to_integer};
use malachite_test::inputs::base::{
    pairs_of_nonempty_unsigned_vec_and_unsigned, pairs_of_signeds,
    pairs_of_unsigned_vec_and_unsigned_var_2, pairs_of_unsigned_vec_var_6,
    pairs_of_unsigned_vec_var_7, triples_of_limb_vec_var_5, triples_of_limb_vec_var_7,
    triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_2,
    triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_3,
};
use malachite_test::inputs::integer::{integers, pairs_of_integers, triples_of_integers};
use malachite_test::inputs::natural::pairs_of_naturals;
use malachite_test::integer::logic::and::{integer_and_alt_1, integer_and_alt_2};

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_pos_and_limb_neg_and_limbs_pos_and_limb_neg_in_place() {
    let test = |limbs: &[Limb], limb: Limb, out: &[Limb]| {
        assert_eq!(limbs_pos_and_limb_neg(limbs, limb), out);

        let mut limbs = limbs.to_vec();
        limbs_pos_and_limb_neg_in_place(&mut limbs, limb);
        assert_eq!(limbs, out);
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
    let test = |out_before: &[Limb], limbs_in: &[Limb], limb: Limb, out_after: &[Limb]| {
        let mut out = out_before.to_vec();
        limbs_pos_and_limb_neg_to_out(&mut out, limbs_in, limb);
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
    let test = |limbs: &[Limb], limb: Limb, out: &[Limb]| {
        assert_eq!(limbs_neg_and_limb_neg(limbs, limb), out);

        let mut limbs = limbs.to_vec();
        limbs_vec_neg_and_limb_neg_in_place(&mut limbs, limb);
        assert_eq!(limbs, out);
    };
    test(&[0, 2], 3, &[0, 2]);
    test(&[1, 1], 3, &[4_294_967_293, 1]);
    test(&[0xffff_fffe, 1], 1, &[0, 2]);
    test(&[0xffff_fffe, 0xffff_ffff], 1, &[0, 0, 1]);
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
    let test = |out_before: &[Limb], in_limbs: &[Limb], limb: Limb, carry, out_after: &[Limb]| {
        let mut out = out_before.to_vec();
        assert_eq!(
            limbs_neg_and_limb_neg_to_out(&mut out, in_limbs, limb),
            carry
        );
        assert_eq!(out, out_after);
    };
    test(&[0, 0], &[0, 2], 3, false, &[0, 2]);
    test(&[1, 2, 100], &[0, 2, 100], 3, false, &[0, 2, 100]);
    test(&[0, 0], &[1, 1], 3, false, &[4_294_967_293, 1]);
    test(&[0, 0], &[0xffff_fffe, 1], 1, false, &[0, 2]);
    test(&[0, 0], &[0xffff_fffe, 0xffff_ffff], 1, true, &[0, 0]);
    test(
        &[1, 2, 100],
        &[0xffff_fffe, 0xffff_ffff],
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
    let test = |limbs_before: &[Limb], limb: Limb, carry, limbs_after: &[Limb]| {
        let mut limbs = limbs_before.to_vec();
        assert_eq!(
            limbs_slice_neg_and_limb_neg_in_place(&mut limbs, limb),
            carry
        );
        assert_eq!(limbs, limbs_after);
    };
    test(&[0, 2], 3, false, &[0, 2]);
    test(&[1, 1], 3, false, &[4_294_967_293, 1]);
    test(&[0xffff_fffe, 1], 1, false, &[0, 2]);
    test(&[0xffff_fffe, 0xffff_ffff], 1, true, &[0, 0]);
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
    let mut limbs = vec![];
    limbs_vec_neg_and_limb_neg_in_place(&mut limbs, 10);
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
        {
            let mut mut_xs = xs.to_vec();
            limbs_vec_and_neg_neg_in_place_left(&mut mut_xs, ys);
            assert_eq!(mut_xs, out);
        }
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
    test(&[0, 2, 1], &[0, 0xffff_ffff, 0xffff_ffff], vec![0, 0, 0, 1]);
    test(&[0, 2], &[0, 0xffff_ffff, 0xffff_ffff], vec![0, 0, 0, 1]);
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
        &[0, 0xffff_ffff],
        &[10, 10, 10, 10],
        false,
        vec![0, 0, 10, 10],
    );
    test(
        &[0, 2, 1],
        &[0, 0xffff_ffff, 0xffff_ffff],
        &[10, 10, 10, 10],
        false,
        vec![0, 0, 0, 10],
    );
    test(
        &[0, 2],
        &[0, 0xffff_ffff, 0xffff_ffff],
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
    test(&[0, 2], &[0, 0xffff_ffff], false, vec![0, 0]);
    test(
        &[0, 2, 1],
        &[0, 0xffff_ffff, 0xffff_ffff],
        false,
        vec![0, 0, 0],
    );
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
        &[0, 0xffff_ffff],
        (false, false),
        vec![0, 0],
        vec![0, 0xffff_ffff],
    );
    test(
        &[0, 2, 1],
        &[0, 0xffff_ffff, 0xffff_ffff],
        (false, false),
        vec![0, 0, 0],
        vec![0, 0xffff_ffff, 0xffff_ffff],
    );
    test(
        &[0, 2],
        &[0, 0xffff_ffff, 0xffff_ffff],
        (true, false),
        vec![0, 2],
        vec![0, 0, 0],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_slice_and_neg_neg_in_place_either_fail_1() {
    limbs_slice_and_neg_neg_in_place_either(&mut vec![0, 0, 0], &mut vec![3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_slice_and_neg_neg_in_place_either_fail_2() {
    limbs_slice_and_neg_neg_in_place_either(&mut vec![3], &mut vec![0, 0, 0]);
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
        &[0, 0xffff_ffff],
        false,
        vec![0, 0, 1],
        vec![0, 0xffff_ffff],
    );
    test(
        &[0, 2, 1],
        &[0, 0xffff_ffff, 0xffff_ffff],
        false,
        vec![0, 0, 0, 1],
        vec![0, 0xffff_ffff, 0xffff_ffff],
    );
    test(
        &[0, 2],
        &[0, 0xffff_ffff, 0xffff_ffff],
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
    let test = |u, v, out| {
        let mut n = Integer::from_str(u).unwrap();
        n &= Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = Integer::from_str(u).unwrap();
        n &= &Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Integer::from_str(u).unwrap() & Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Integer::from_str(u).unwrap() & Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Integer::from_str(u).unwrap() & &Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Integer::from_str(u).unwrap() & &Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        assert_eq!(
            integer_and_alt_1(
                &Integer::from_str(u).unwrap(),
                &Integer::from_str(v).unwrap(),
            )
            .to_string(),
            out
        );
        assert_eq!(
            integer_and_alt_2(
                &Integer::from_str(u).unwrap(),
                &Integer::from_str(v).unwrap(),
            )
            .to_string(),
            out
        );

        let n = rug::Integer::from_str(u).unwrap() & rug::Integer::from_str(v).unwrap();
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
    test_properties(
        pairs_of_nonempty_unsigned_vec_and_unsigned,
        |&(ref limbs, limb)| {
            let out = limbs_pos_and_limb_neg(limbs, limb);
            let n = Integer::from(Natural::from_limbs_asc(limbs))
                & Integer::from_owned_twos_complement_limbs_asc(vec![limb, Limb::MAX]);
            assert_eq!(Natural::from_owned_limbs_asc(out), Natural::exact_from(n));
        },
    );
}

#[test]
fn limbs_pos_and_limb_neg_to_out_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_2,
        |&(ref out, ref in_limbs, limb)| {
            let mut out = out.to_vec();
            let old_out = out.clone();
            limbs_pos_and_limb_neg_to_out(&mut out, in_limbs, limb);
            let n = Integer::from(Natural::from_limbs_asc(in_limbs))
                & Integer::from_owned_twos_complement_limbs_asc(vec![limb, Limb::MAX]);
            let len = in_limbs.len();
            let mut limbs = Natural::exact_from(n).into_limbs_asc();
            limbs.resize(len, 0);
            assert_eq!(limbs, &out[..len]);
            assert_eq!(&out[len..], &old_out[len..]);
        },
    );
}

#[test]
fn limbs_pos_and_limb_neg_in_place_properties() {
    test_properties(
        pairs_of_nonempty_unsigned_vec_and_unsigned,
        |&(ref limbs, limb)| {
            let mut limbs = limbs.to_vec();
            limbs_pos_and_limb_neg_in_place(&mut limbs, limb);
            let n = Integer::from(Natural::from_limbs_asc(&limbs))
                & Integer::from_owned_twos_complement_limbs_asc(vec![limb, Limb::MAX]);
            assert_eq!(Natural::from_owned_limbs_asc(limbs), Natural::exact_from(n));
        },
    );
}

#[test]
fn limbs_neg_and_limb_neg_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_unsigned_var_2,
        |&(ref limbs, limb)| {
            let out = limbs_neg_and_limb_neg(limbs, limb);
            let n = -Natural::from_limbs_asc(limbs)
                & Integer::from_owned_twos_complement_limbs_asc(vec![limb, Limb::MAX]);
            assert_eq!(Natural::from_owned_limbs_asc(out), Natural::exact_from(-n));
        },
    );
}

#[test]
fn limbs_neg_and_limb_neg_to_out_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_3,
        |&(ref out, ref in_limbs, limb)| {
            let mut out = out.to_vec();
            let old_out = out.clone();
            let carry = limbs_neg_and_limb_neg_to_out(&mut out, in_limbs, limb);
            let n = -Natural::from_limbs_asc(in_limbs)
                & Integer::from_owned_twos_complement_limbs_asc(vec![limb, Limb::MAX]);
            let len = in_limbs.len();
            let mut limbs = Natural::exact_from(-n).into_limbs_asc();
            assert_eq!(carry, limbs.len() == len + 1);
            limbs.resize(len, 0);
            assert_eq!(limbs, &out[..len]);
            assert_eq!(&out[len..], &old_out[len..]);
        },
    );
}

#[test]
fn limbs_slice_neg_and_limb_neg_in_place_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_unsigned_var_2,
        |&(ref limbs, limb)| {
            let mut limbs = limbs.to_vec();
            let old_limbs = limbs.clone();
            limbs_slice_neg_and_limb_neg_in_place(&mut limbs, limb);
            let n = -Natural::from_limbs_asc(&old_limbs)
                & Integer::from_owned_twos_complement_limbs_asc(vec![limb, Limb::MAX]);
            let mut expected_limbs = Natural::exact_from(-n).into_limbs_asc();
            expected_limbs.resize(limbs.len(), 0);
            assert_eq!(limbs, expected_limbs);
        },
    );
}

#[test]
fn limbs_vec_neg_and_limb_neg_in_place_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_unsigned_var_2,
        |&(ref limbs, limb)| {
            let mut limbs = limbs.to_vec();
            limbs_vec_neg_and_limb_neg_in_place(&mut limbs, limb);
            let n = -Natural::from_limbs_asc(&limbs)
                & Integer::from_owned_twos_complement_limbs_asc(vec![limb, Limb::MAX]);
            assert_eq!(
                Natural::from_owned_limbs_asc(limbs),
                Natural::exact_from(-n)
            );
        },
    );
}

#[test]
fn limbs_and_pos_neg_properties() {
    test_properties(pairs_of_unsigned_vec_var_6, |&(ref xs, ref ys)| {
        assert_eq!(
            Natural::from_owned_limbs_asc(limbs_and_pos_neg(xs, ys)),
            Integer::from(Natural::from_limbs_asc(xs)) & -Natural::from_limbs_asc(ys)
        );
    });
}

#[test]
fn limbs_and_pos_neg_to_out_properties() {
    test_properties(triples_of_limb_vec_var_5, |&(ref out, ref xs, ref ys)| {
        let mut out = out.to_vec();
        let out_old = out.clone();
        limbs_and_pos_neg_to_out(&mut out, xs, ys);
        let len = xs.len();
        assert_eq!(
            Natural::from_limbs_asc(&out[..len]),
            Integer::from(Natural::from_limbs_asc(xs)) & -Natural::from_limbs_asc(ys)
        );
        assert_eq!(&out[len..], &out_old[len..]);
    });
}

#[test]
fn limbs_and_pos_neg_in_place_left_properties() {
    test_properties(pairs_of_unsigned_vec_var_6, |&(ref xs, ref ys)| {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        limbs_and_pos_neg_in_place_left(&mut xs, ys);
        assert_eq!(
            Natural::from_owned_limbs_asc(xs),
            Integer::from(Natural::from_owned_limbs_asc(xs_old)) & -Natural::from_limbs_asc(ys)
        );
    });
}

#[test]
fn limbs_slice_and_pos_neg_in_place_right_properties() {
    test_properties(pairs_of_unsigned_vec_var_6, |&(ref xs, ref ys)| {
        let mut ys = ys.to_vec();
        let ys_old = ys.clone();
        limbs_slice_and_pos_neg_in_place_right(xs, &mut ys);
        let result =
            Integer::from(Natural::from_limbs_asc(xs)) & -Natural::from_owned_limbs_asc(ys_old);
        let mut expected_limbs = Natural::exact_from(result).into_limbs_asc();
        let len = min(xs.len(), ys.len());
        expected_limbs.resize(len, 0);
        assert_eq!(&ys[..len], expected_limbs.as_slice());
    });
}

#[test]
fn limbs_vec_and_pos_neg_in_place_right_properties() {
    test_properties(pairs_of_unsigned_vec_var_6, |&(ref xs, ref ys)| {
        let mut ys = ys.to_vec();
        let ys_old = ys.clone();
        limbs_vec_and_pos_neg_in_place_right(xs, &mut ys);
        let result =
            Integer::from(Natural::from_limbs_asc(xs)) & -Natural::from_owned_limbs_asc(ys_old);
        let mut expected_limbs = Natural::exact_from(result).into_limbs_asc();
        expected_limbs.resize(xs.len(), 0);
        ys.resize(xs.len(), 0);
        assert_eq!(ys, expected_limbs);
    });
}

#[test]
fn limbs_and_neg_neg_properties() {
    test_properties(pairs_of_unsigned_vec_var_6, |&(ref xs, ref ys)| {
        assert_eq!(
            -Natural::from_owned_limbs_asc(limbs_and_neg_neg(xs, ys)),
            -Natural::from_limbs_asc(xs) & -Natural::from_limbs_asc(ys)
        );
    });
}

#[test]
fn limbs_and_neg_neg_to_out_properties() {
    test_properties(triples_of_limb_vec_var_7, |&(ref xs, ref ys, ref zs)| {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        let b = limbs_and_neg_neg_to_out(&mut xs, ys, zs);
        let len = max(ys.len(), zs.len());
        let result =
            Natural::exact_from(-(-Natural::from_limbs_asc(ys) & -Natural::from_limbs_asc(zs)));
        let mut expected_limbs = result.to_limbs_asc();
        expected_limbs.resize(len, 0);
        assert_eq!(b, Natural::from_limbs_asc(&expected_limbs) == result);
        assert_eq!(&xs[..len], expected_limbs.as_slice());
        assert_eq!(&xs[len..], &xs_old[len..]);
    });
}

#[test]
fn limbs_slice_and_neg_neg_in_place_left_properties() {
    test_properties(pairs_of_unsigned_vec_var_7, |&(ref xs, ref ys)| {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        let b = limbs_slice_and_neg_neg_in_place_left(&mut xs, ys);
        let len = xs_old.len();
        let result = Natural::exact_from(
            -(-Natural::from_owned_limbs_asc(xs_old) & -Natural::from_limbs_asc(ys)),
        );
        let mut expected_limbs = result.to_limbs_asc();
        expected_limbs.resize(len, 0);
        assert_eq!(b, Natural::from_limbs_asc(&expected_limbs) == result);
        assert_eq!(xs, expected_limbs.as_slice());
    });
}

#[test]
fn limbs_vec_and_neg_neg_in_place_left_properties() {
    test_properties(pairs_of_unsigned_vec_var_6, |&(ref xs, ref ys)| {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        limbs_vec_and_neg_neg_in_place_left(&mut xs, ys);
        assert_eq!(
            -Natural::from_owned_limbs_asc(xs),
            -Natural::from_owned_limbs_asc(xs_old) & -Natural::from_limbs_asc(ys)
        );
    });
}

#[test]
fn limbs_slice_and_neg_neg_in_place_either_properties() {
    test_properties(pairs_of_unsigned_vec_var_6, |&(ref xs, ref ys)| {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        let mut ys = ys.to_vec();
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
    test_properties(pairs_of_unsigned_vec_var_6, |&(ref xs, ref ys)| {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        let mut ys = ys.to_vec();
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

#[test]
fn and_properties() {
    test_properties(pairs_of_integers, |&(ref x, ref y)| {
        let result_val_val = x.clone() & y.clone();
        let result_val_ref = x.clone() & y;
        let result_ref_val = x & y.clone();
        let result = x & y;
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
        mut_x &= y;
        assert_eq!(mut_x, result);
        assert!(mut_x.is_valid());

        let mut mut_x = integer_to_rug_integer(x);
        mut_x &= integer_to_rug_integer(y);
        assert_eq!(rug_integer_to_integer(&mut_x), result);

        assert_eq!(
            rug_integer_to_integer(&(integer_to_rug_integer(x) & integer_to_rug_integer(y))),
            result
        );

        assert_eq!(integer_and_alt_1(&x, y), result);
        assert_eq!(integer_and_alt_2(&x, y), result);

        assert_eq!(y & x, result);
        assert_eq!(&result & x, result);
        assert_eq!(&result & y, result);
        assert_eq!(!(!x | !y), result);
    });

    test_properties(integers, |x| {
        assert_eq!(x & Integer::ZERO, 0);
        assert_eq!(Integer::ZERO & x, 0);
        assert_eq!(x & x, *x);
        assert_eq!(x & !x, 0);
    });

    test_properties(triples_of_integers, |&(ref x, ref y, ref z)| {
        assert_eq!((x & y) & z, x & (y & z));
    });

    test_properties(pairs_of_signeds::<SignedLimb>, |&(i, j)| {
        assert_eq!(Integer::from(i) & Integer::from(j), i & j);
    });

    test_properties(pairs_of_naturals, |&(ref x, ref y)| {
        assert_eq!(Integer::from(x) & Integer::from(y), x & y);
    });
}
