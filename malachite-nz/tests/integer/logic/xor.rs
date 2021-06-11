use malachite_nz_test_util::integer::logic::xor::{integer_xor_alt_1, integer_xor_alt_2};
use rug;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
use malachite_nz::integer::logic::xor::{
    limbs_neg_xor_limb, limbs_neg_xor_limb_neg, limbs_neg_xor_limb_neg_in_place,
    limbs_neg_xor_limb_neg_to_out, limbs_neg_xor_limb_to_out, limbs_pos_xor_limb_neg,
    limbs_pos_xor_limb_neg_to_out, limbs_slice_pos_xor_limb_neg_in_place,
    limbs_vec_neg_xor_limb_in_place, limbs_vec_pos_xor_limb_neg_in_place, limbs_xor_neg_neg,
    limbs_xor_neg_neg_in_place_either, limbs_xor_neg_neg_in_place_left, limbs_xor_neg_neg_to_out,
    limbs_xor_pos_neg, limbs_xor_pos_neg_in_place_either, limbs_xor_pos_neg_in_place_left,
    limbs_xor_pos_neg_in_place_right, limbs_xor_pos_neg_to_out,
};
use malachite_nz::integer::Integer;
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::platform::Limb;

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
    test(&[u32::MAX - 1, u32::MAX, u32::MAX], 2, &[0, 0, 0, 1]);
    test(&[0, 0, 0, 1], 2, &[u32::MAX - 1, u32::MAX, u32::MAX, 0]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_neg_xor_limb_to_out() {
    let test = |out_before: &[Limb], xs: &[Limb], y: Limb, carry, out_after: &[Limb]| {
        let mut out = out_before.to_vec();
        assert_eq!(limbs_neg_xor_limb_to_out(&mut out, xs, y), carry);
        assert_eq!(out, out_after);
    };
    test(&[10, 10, 10, 10], &[6, 7], 0, false, &[6, 7, 10, 10]);
    test(&[10, 10, 10, 10], &[6, 7], 2, false, &[8, 7, 10, 10]);
    test(
        &[10, 10, 10, 10],
        &[100, 101, 102],
        10,
        false,
        &[106, 101, 102, 10],
    );
    test(
        &[10, 10, 10, 10],
        &[123, 456],
        789,
        false,
        &[880, 456, 10, 10],
    );
    test(
        &[10, 10, 10, 10],
        &[u32::MAX - 1, u32::MAX, u32::MAX],
        2,
        true,
        &[0, 0, 0, 10],
    );
    test(
        &[10, 10, 10, 10],
        &[0, 0, 0, 1],
        2,
        false,
        &[u32::MAX - 1, u32::MAX, u32::MAX, 0],
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
    test(&[2, u32::MAX], 2, &[0, 0, 1]);
    test(&[2, u32::MAX, u32::MAX], 2, &[0, 0, 0, 1]);
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
    test(&[0, 0], &[2, u32::MAX], 2, true, &[0, 0]);
    test(&[0, 0, 0], &[2, u32::MAX, u32::MAX], 2, true, &[0, 0, 0]);
    test(
        &[1, 2, 3, 100],
        &[2, u32::MAX, u32::MAX],
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
    test(&[2, u32::MAX], 2, true, &[0, 0]);
    test(&[2, u32::MAX, u32::MAX], 2, true, &[0, 0, 0]);
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
    test(&[10, 10, 10, 10], &[0, 2], 3, &[3, 1, 10, 10]);
    test(&[10, 10, 10, 10], &[6, 7], 2, &[0xfffffff8, 7, 10, 10]);
    test(&[10, 10, 10, 10], &[1, 2, 3], 4, &[0xfffffffb, 2, 3, 10]);
    test(
        &[10, 10, 10, 10],
        &[100, 101, 102],
        10,
        &[4294967190, 101, 102, 10],
    );
    test(
        &[10, 10, 10, 10],
        &[123, 456],
        789,
        &[4294966416, 456, 10, 10],
    );
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
    test(&[3], &[0, 0, 1], vec![0xfffffffd, u32::MAX, 0]);
    test(&[0, 3, 3], &[0, 0, 3], vec![0, 0xfffffffd, 1]);
    test(&[0, 0, 3], &[0, 3, 3], vec![0, 3, 0]);
    test(&[0, 3], &[0, 0, 3], vec![0, 0xfffffffd, 2]);
    test(&[0, 0, 3], &[0, 3], vec![0, 3, 3]);
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
    let test = |xs, ys, out_before: &[Limb], out_after| {
        let mut out = out_before.to_vec();
        limbs_xor_pos_neg_to_out(&mut out, xs, ys);
        assert_eq!(out, out_after);
    };
    test(&[2], &[3], &[10, 10, 10, 10], vec![1, 10, 10, 10]);
    test(&[1, 1, 1], &[1, 2, 3], &[10, 10, 10, 10], vec![2, 3, 2, 10]);
    test(&[6, 7], &[1, 2, 3], &[10, 10, 10, 10], vec![7, 5, 3, 10]);
    test(&[1, 2, 3], &[6, 7], &[10, 10, 10, 10], vec![5, 5, 3, 10]);
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        &[10, 10, 10, 10],
        vec![2, 0, 2, 10],
    );
    test(&[0, 0, 1], &[3], &[10, 10, 10, 10], vec![3, 0, 1, 10]);
    test(
        &[3],
        &[0, 0, 1],
        &[10, 10, 10, 10],
        vec![0xfffffffd, u32::MAX, 0, 10],
    );
    test(
        &[0, 3, 3],
        &[0, 0, 3],
        &[10, 10, 10, 10],
        vec![0, 0xfffffffd, 1, 10],
    );
    test(&[0, 0, 3], &[0, 3, 3], &[10, 10, 10, 10], vec![0, 3, 0, 10]);
    test(
        &[0, 3],
        &[0, 0, 3],
        &[10, 10, 10, 10],
        vec![0, 0xfffffffd, 2, 10],
    );
    test(&[0, 0, 3], &[0, 3], &[10, 10, 10, 10], vec![0, 3, 3, 10]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_xor_pos_neg_to_out_fail_1() {
    let mut out = vec![10, 10, 10, 10];
    limbs_xor_pos_neg_to_out(&mut out, &[0, 0, 0], &[3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_xor_pos_neg_to_out_fail_2() {
    let mut out = vec![10, 10, 10, 10];
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
        vec![0xfffffffd, u32::MAX, 0],
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
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_xor_pos_neg_in_place_either_fail_1() {
    limbs_xor_pos_neg_in_place_either(&mut [0, 0, 0], &mut [3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_xor_pos_neg_in_place_either_fail_2() {
    limbs_xor_pos_neg_in_place_either(&mut [3], &mut [0, 0, 0]);
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
    test(&[0, 0, 1], &[3], vec![0xfffffffd, u32::MAX, 0]);
    test(&[3], &[0, 0, 1], vec![0xfffffffd, u32::MAX, 0]);
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
    test(&[2], &[3], &[10, 10, 10, 10], vec![3, 10, 10, 10]);
    test(&[1, 1, 1], &[1, 2, 3], &[10, 10, 10, 10], vec![0, 3, 2, 10]);
    test(&[6, 7], &[1, 2, 3], &[10, 10, 10, 10], vec![5, 5, 3, 10]);
    test(&[1, 2, 3], &[6, 7], &[10, 10, 10, 10], vec![5, 5, 3, 10]);
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        &[10, 10, 10, 10],
        vec![6, 0, 2, 10],
    );
    test(
        &[0, 0, 1],
        &[3],
        &[10, 10, 10, 10],
        vec![0xfffffffd, u32::MAX, 0, 10],
    );
    test(
        &[3],
        &[0, 0, 1],
        &[10, 10, 10, 10],
        vec![0xfffffffd, u32::MAX, 0, 10],
    );
    test(
        &[0, 3, 3],
        &[0, 0, 3],
        &[10, 10, 10, 10],
        vec![0, 0xfffffffd, 1, 10],
    );
    test(
        &[0, 0, 3],
        &[0, 3, 3],
        &[10, 10, 10, 10],
        vec![0, 0xfffffffd, 1, 10],
    );
    test(
        &[0, 3],
        &[0, 0, 3],
        &[10, 10, 10, 10],
        vec![0, 0xfffffffd, 2, 10],
    );
    test(
        &[0, 0, 3],
        &[0, 3],
        &[10, 10, 10, 10],
        vec![0, 0xfffffffd, 2, 10],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_xor_neg_neg_to_out_fail_1() {
    let mut out = vec![10, 10, 10, 10];
    limbs_xor_neg_neg_to_out(&mut out, &[0, 0, 0], &[3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_xor_neg_neg_to_out_fail_2() {
    let mut out = vec![10, 10, 10, 10];
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
        vec![0xfffffffd, u32::MAX, 0],
        vec![3],
    );
    test(
        &[3],
        &[0, 0, 1],
        true,
        vec![3],
        vec![0xfffffffd, u32::MAX, 0],
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

        assert_eq!(integer_xor_alt_1(&u, &v,).to_string(), out);
        assert_eq!(integer_xor_alt_2(&u, &v,).to_string(), out);

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
}
