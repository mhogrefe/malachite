use std::str::FromStr;

use malachite_nz_test_util::integer::logic::or::{integer_or_alt_1, integer_or_alt_2};
use rug;

#[cfg(feature = "32_bit_limbs")]
use malachite_nz::integer::logic::or::{
    limbs_neg_or_limb, limbs_neg_or_limb_in_place, limbs_neg_or_limb_to_out, limbs_neg_or_neg_limb,
    limbs_or_neg_neg, limbs_or_neg_neg_in_place_either, limbs_or_neg_neg_to_out, limbs_or_pos_neg,
    limbs_or_pos_neg_in_place_right, limbs_or_pos_neg_to_out, limbs_pos_or_neg_limb,
    limbs_slice_or_neg_neg_in_place_left, limbs_slice_or_pos_neg_in_place_left,
    limbs_vec_or_neg_neg_in_place_left, limbs_vec_or_pos_neg_in_place_left,
};
use malachite_nz::integer::Integer;
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::platform::Limb;

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
    test(&[0, 0, 456], 789, &[0xffff_fceb, u32::MAX, 455]);
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
        &[0xffff_fceb, u32::MAX, 455, 10],
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
    test(&[6, 7], 3, 0xffff_fff9);
    test(&[100, 101, 102], 10, 0xffff_ff92);
    test(&[0, 0, 1], 100, 0xffff_ff9c);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pos_or_neg_limb_fail() {
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
    test(&[0, 0, 1], 100, 0xffff_ff9c);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_neg_or_neg_limb_fail() {
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
    test(&[3], &[0, 0, 1], vec![0xffff_fffd, u32::MAX, 0]);
    test(&[0, 3, 3], &[0, 0, 3], vec![0, 0xffff_fffd, 0]);
    test(&[0, 0, 3], &[0, 3, 3], vec![0, 3, 0]);
    test(&[0, 3], &[0, 0, 3], vec![0, 0xffff_fffd, 2]);
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
        vec![0xffff_fffd, u32::MAX, 0, 10],
    );
    test(
        &[0, 3, 3],
        &[0, 0, 3],
        &[10, 10, 10, 10],
        vec![0, 0xffff_fffd, 0, 10],
    );
    test(&[0, 0, 3], &[0, 3, 3], &[10, 10, 10, 10], vec![0, 3, 0, 10]);
    test(
        &[0, 3],
        &[0, 0, 3],
        &[10, 10, 10, 10],
        vec![0, 0xffff_fffd, 2, 10],
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
    test(&[3], &[0, 0, 1], true, vec![0xffff_fffd]);
    test(&[0, 3, 3], &[0, 0, 3], false, vec![0, 0xffff_fffd, 0]);
    test(&[0, 0, 3], &[0, 3, 3], false, vec![0, 3, 0]);
    test(&[0, 3], &[0, 0, 3], true, vec![0, 0xffff_fffd]);
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
    let test = |u, v, out| {
        let mut n = Integer::from_str(u).unwrap();
        n |= Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = Integer::from_str(u).unwrap();
        n |= &Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Integer::from_str(u).unwrap() | Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Integer::from_str(u).unwrap() | Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Integer::from_str(u).unwrap() | &Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Integer::from_str(u).unwrap() | &Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        assert_eq!(
            integer_or_alt_1(
                &Integer::from_str(u).unwrap(),
                &Integer::from_str(v).unwrap(),
            )
            .to_string(),
            out
        );
        assert_eq!(
            integer_or_alt_2(
                &Integer::from_str(u).unwrap(),
                &Integer::from_str(v).unwrap(),
            )
            .to_string(),
            out
        );

        let n = rug::Integer::from_str(u).unwrap() | rug::Integer::from_str(v).unwrap();
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
