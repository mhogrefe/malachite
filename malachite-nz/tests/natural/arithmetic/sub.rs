use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
use malachite_nz_test_util::natural::arithmetic::sub::{
    limbs_sub_same_length_in_place_with_overlap_naive,
    limbs_sub_same_length_to_out_with_overlap_naive,
};
use num::BigUint;
use rug;

#[cfg(feature = "32_bit_limbs")]
use malachite_nz::natural::arithmetic::sub::{
    _limbs_sub_same_length_with_borrow_in_in_place_left,
    _limbs_sub_same_length_with_borrow_in_to_out, limbs_slice_sub_in_place_right, limbs_sub,
    limbs_sub_in_place_left, limbs_sub_limb, limbs_sub_limb_in_place, limbs_sub_limb_to_out,
    limbs_sub_same_length_in_place_left, limbs_sub_same_length_in_place_right,
    limbs_sub_same_length_in_place_with_overlap, limbs_sub_same_length_to_out,
    limbs_sub_same_length_to_out_with_overlap, limbs_sub_to_out, limbs_vec_sub_in_place_right,
};
use malachite_nz::natural::Natural;
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::platform::Limb;

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
    test(&[123, 456], 789, (vec![4_294_966_630, 455], false));
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
        &[4_294_966_630, 455, 10, 10],
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
    test(&[123, 456], 789, false, &[4_294_966_630, 455]);
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
        (vec![0, u32::MAX, 0xffff_fffd], true),
    );
    test(
        &[1, 2, 3],
        &[6, 7],
        (vec![0xffff_fffb, 0xffff_fffa, 2], false),
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
        vec![0, u32::MAX, 0xffff_fffd, 5],
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
        &[u32::MAX, u32::MAX, 0xff_ffff, 0, 0],
        &[0, 0, 0, 4_294_967_232, u32::MAX],
        &[10, 10, 10, 10, 10, 10],
        true,
        vec![u32::MAX, u32::MAX, 0xff_ffff, 64, 0, 10],
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
fn test_limbs_sub_to_out() {
    let test = |xs, ys, out_before: &[Limb], borrow, out_after| {
        let mut out = out_before.to_vec();
        assert_eq!(limbs_sub_to_out(&mut out, xs, ys), borrow);
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
        vec![0, u32::MAX, 0xffff_fffd, 5],
    );
    test(
        &[1, 2, 3],
        &[6, 7],
        &[0, 0, 0],
        false,
        vec![0xffff_fffb, 0xffff_fffa, 2],
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
fn limbs_sub_to_out_fail_1() {
    let mut out = vec![10, 10];
    limbs_sub_to_out(&mut out, &[6, 7, 8], &[1, 2]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_to_out_fail_2() {
    let mut out = vec![10, 10, 10];
    limbs_sub_to_out(&mut out, &[6, 7], &[1, 2, 3]);
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
    test(&[1, 1, 1], &[1, 2, 3], true, vec![0, u32::MAX, 0xffff_fffd]);
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
fn test_limbs_sub_in_place_left() {
    let test = |xs_before: &[Limb], ys, borrow, xs_after| {
        let mut xs = xs_before.to_vec();
        assert_eq!(limbs_sub_in_place_left(&mut xs, ys), borrow);
        assert_eq!(xs, xs_after);
    };
    test(&[], &[], false, vec![]);
    test(&[2], &[], false, vec![2]);
    test(&[3], &[2], false, vec![1]);
    test(&[2], &[3], true, vec![u32::MAX]);
    test(&[1, 2, 3], &[1, 1, 1], false, vec![0, 1, 2]);
    test(&[1, 1, 1], &[1, 2, 3], true, vec![0, u32::MAX, 0xffff_fffd]);
    test(
        &[1, 2, 3],
        &[6, 7],
        false,
        vec![0xffff_fffb, 0xffff_fffa, 2],
    );
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
fn limbs_sub_in_place_left_fail() {
    limbs_sub_in_place_left(&mut [6, 7], &[1, 2, 3]);
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
    test(&[1, 1, 1], &[1, 2, 3], true, vec![0, u32::MAX, 0xffff_fffd]);
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
    test(&[123, 456], &[789, 123], 2, false, &[4_294_966_630, 332]);
    test(&[123, 456], &[789, 123], 1, false, &[4_294_966_630, 455]);
    test(&[123, 0], &[789, 123], 1, true, &[4_294_966_630, u32::MAX]);
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
    test(&[1, 1, 1], &[1, 2, 3], true, vec![0, u32::MAX, 0xffff_fffd]);
    test(
        &[1, 2, 3],
        &[6, 7],
        false,
        vec![0xffff_fffb, 0xffff_fffa, 2],
    );
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
            _limbs_sub_same_length_with_borrow_in_to_out(&mut out, xs, ys, borrow_in),
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
        vec![0, u32::MAX, 0xffff_fffd, 5],
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
    _limbs_sub_same_length_with_borrow_in_to_out(&mut out, &[6, 7, 8], &[1, 2, 3], false);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_same_length_with_borrow_in_to_out_fail_2() {
    let mut out = vec![10, 10, 10];
    _limbs_sub_same_length_with_borrow_in_to_out(&mut out, &[6, 7, 8], &[1, 2], false);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_sub_same_length_with_borrow_in_in_place_left() {
    let test = |xs_before: &[Limb], ys, borrow_in, borrow, xs_after| {
        let mut xs = xs_before.to_vec();
        assert_eq!(
            _limbs_sub_same_length_with_borrow_in_in_place_left(&mut xs, ys, borrow_in),
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
        vec![0, u32::MAX, 0xffff_fffd],
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
    _limbs_sub_same_length_with_borrow_in_in_place_left(&mut [6, 7], &[1, 2, 3], false);
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
    test(&[1, 2, 3, 4], 2, true, &[u32::MAX - 1, 0xffff_fffd, 3, 4]);
    test(&[1, 2, 3, 4], 3, true, &[0xffff_fffd, 2, 3, 4]);
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
        &[0xffff_fffd, 0xffff_fffd, u32::MAX - 1, u32::MAX],
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
    let test = |u, v, out| {
        let mut n = Natural::from_str(u).unwrap();
        n -= Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = Natural::from_str(u).unwrap();
        n -= &Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u).unwrap() - Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u).unwrap() - &Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Natural::from_str(u).unwrap() - Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Natural::from_str(u).unwrap() - &Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = BigUint::from_str(u).unwrap() - BigUint::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);

        let n = rug::Integer::from_str(u).unwrap() - rug::Integer::from_str(v).unwrap();
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
