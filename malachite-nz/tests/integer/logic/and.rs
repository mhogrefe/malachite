use std::str::FromStr;

use malachite_nz_test_util::integer::logic::and::{integer_and_alt_1, integer_and_alt_2};
use rug;

#[cfg(feature = "32_bit_limbs")]
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
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::platform::Limb;

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
    test(&[1, 1], 3, &[0xffff_fffd, 1]);
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
    test(&[0, 0], &[1, 1], 3, false, &[0xffff_fffd, 1]);
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
    test(&[1, 1], 3, false, &[0xffff_fffd, 1]);
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
