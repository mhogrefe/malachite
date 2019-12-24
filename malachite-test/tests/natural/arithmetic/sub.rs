use std::str::FromStr;

use malachite_base::comparison::Max;
use malachite_base::crement::Crementable;
use malachite_base::limbs::limbs_test_zero;
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::{CheckedFrom, WrappingFrom};
use malachite_nz::integer::Integer;
use malachite_nz::natural::arithmetic::sub::{
    _limbs_sub_same_length_with_borrow_in_in_place_left,
    _limbs_sub_same_length_with_borrow_in_to_out, limbs_slice_sub_in_place_right, limbs_sub,
    limbs_sub_in_place_left, limbs_sub_limb, limbs_sub_limb_in_place, limbs_sub_limb_to_out,
    limbs_sub_same_length_in_place_left, limbs_sub_same_length_in_place_right,
    limbs_sub_same_length_in_place_with_overlap, limbs_sub_same_length_to_out,
    limbs_sub_same_length_to_out_with_overlap, limbs_sub_to_out, limbs_vec_sub_in_place_right,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use num::BigUint;
use rug;

use malachite_test::common::test_properties;
use malachite_test::common::{
    biguint_to_natural, natural_to_biguint, natural_to_rug_integer, rug_integer_to_natural,
};
use malachite_test::inputs::base::pairs_of_unsigneds_var_1;
use malachite_test::inputs::base::{
    pairs_of_unsigned_vec_and_small_usize_var_1, pairs_of_unsigned_vec_and_unsigned,
    pairs_of_unsigned_vec_var_1, pairs_of_unsigned_vec_var_3,
    quadruples_of_three_unsigned_vecs_and_bool_var_1, triples_of_two_unsigned_vecs_and_bool_var_1,
    triples_of_unsigned_vec_unsigned_and_small_usize_var_1,
    triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_1, triples_of_unsigned_vec_var_3,
    triples_of_unsigned_vec_var_9, vecs_of_unsigned,
};
use malachite_test::inputs::natural::{naturals, pairs_of_naturals_var_1};
use malachite_test::natural::arithmetic::sub::{
    limbs_sub_same_length_in_place_with_overlap_naive,
    limbs_sub_same_length_to_out_with_overlap_naive,
};

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_sub_limb() {
    let test = |limbs: &[Limb], limb: Limb, out: (Vec<Limb>, bool)| {
        assert_eq!(limbs_sub_limb(limbs, limb), out);
    };
    test(&[], 0, (vec![], false));
    test(&[1], 2, (vec![4_294_967_295], true));
    test(&[6, 7], 2, (vec![4, 7], false));
    test(&[100, 101, 102], 10, (vec![90, 101, 102], false));
    test(&[123, 456], 78, (vec![45, 456], false));
    test(&[123, 456], 789, (vec![4_294_966_630, 455], false));
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_sub_limb_to_out() {
    let test =
        |out_before: &[Limb], limbs_in: &[Limb], limb: Limb, borrow: bool, out_after: &[Limb]| {
            let mut out = out_before.to_vec();
            assert_eq!(limbs_sub_limb_to_out(&mut out, limbs_in, limb), borrow);
            assert_eq!(out, out_after);
        };
    test(&[10, 10, 10, 10], &[], 0, false, &[10, 10, 10, 10]);
    test(
        &[10, 10, 10, 10],
        &[1],
        2,
        true,
        &[4_294_967_295, 10, 10, 10],
    );
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
    let test = |limbs: &[Limb], limb: Limb, borrow: bool, out: &[Limb]| {
        let mut limbs = limbs.to_vec();
        assert_eq!(limbs_sub_limb_in_place(&mut limbs, limb), borrow);
        assert_eq!(limbs, out);
    };
    test(&[], 0, false, &[]);
    test(&[1], 2, true, &[4_294_967_295]);
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
    test(&[2], &[3], (vec![4_294_967_295], true));
    test(&[1, 2, 3], &[1, 1, 1], (vec![0, 1, 2], false));
    test(
        &[1, 1, 1],
        &[1, 2, 3],
        (vec![0, 4_294_967_295, 4_294_967_293], true),
    );
    test(
        &[1, 2, 3],
        &[6, 7],
        (vec![4_294_967_291, 4_294_967_290, 2], false),
    );
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        (vec![4_294_967_294, 4_294_967_295, 1], false),
    );
    test(
        &[102, 101, 100],
        &[100, 101, 102],
        (vec![2, 0, 4_294_967_294], true),
    );
    test(
        &[0, 0, 0],
        &[1],
        (vec![4_294_967_295, 4_294_967_295, 4_294_967_295], true),
    );
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
    test(&[2], &[3], &[0, 0], true, vec![4_294_967_295, 0]);
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
        vec![0, 4_294_967_295, 4_294_967_293, 5],
    );
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        &[10, 10, 10, 10],
        false,
        vec![4_294_967_294, 4_294_967_295, 1, 10],
    );
    test(
        &[102, 101, 100],
        &[100, 101, 102],
        &[10, 10, 10, 10],
        true,
        vec![2, 0, 4_294_967_294, 10],
    );
    test(
        &[0, 0, 0],
        &[1, 0, 0],
        &[10, 10, 10, 10],
        true,
        vec![4_294_967_295, 4_294_967_295, 4_294_967_295, 10],
    );
    test(
        &[4294967295, 4294967295, 16777215, 0, 0],
        &[0, 0, 0, 4294967232, 4294967295],
        &[10, 10, 10, 10, 10, 10],
        true,
        vec![4294967295, 4294967295, 16777215, 64, 0, 10],
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
    test(&[2], &[3], &[0, 0], true, vec![4_294_967_295, 0]);
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
        vec![0, 4_294_967_295, 4_294_967_293, 5],
    );
    test(
        &[1, 2, 3],
        &[6, 7],
        &[0, 0, 0],
        false,
        vec![4_294_967_291, 4_294_967_290, 2],
    );
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        &[10, 10, 10, 10],
        false,
        vec![4_294_967_294, 4_294_967_295, 1, 10],
    );
    test(
        &[102, 101, 100],
        &[100, 101, 102],
        &[10, 10, 10, 10],
        true,
        vec![2, 0, 4_294_967_294, 10],
    );
    test(
        &[0, 0, 0],
        &[1],
        &[10, 10, 10, 10],
        true,
        vec![4_294_967_295, 4_294_967_295, 4_294_967_295, 10],
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
    test(&[2], &[3], true, vec![4_294_967_295]);
    test(&[1, 2, 3], &[1, 1, 1], false, vec![0, 1, 2]);
    test(
        &[1, 1, 1],
        &[1, 2, 3],
        true,
        vec![0, 4_294_967_295, 4_294_967_293],
    );
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        false,
        vec![4_294_967_294, 4_294_967_295, 1],
    );
    test(
        &[102, 101, 100],
        &[100, 101, 102],
        true,
        vec![2, 0, 4_294_967_294],
    );
    test(
        &[0, 0, 0],
        &[1, 0, 0],
        true,
        vec![4_294_967_295, 4_294_967_295, 4_294_967_295],
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
    test(&[2], &[3], true, vec![4_294_967_295]);
    test(&[1, 2, 3], &[1, 1, 1], false, vec![0, 1, 2]);
    test(
        &[1, 1, 1],
        &[1, 2, 3],
        true,
        vec![0, 4_294_967_295, 4_294_967_293],
    );
    test(
        &[1, 2, 3],
        &[6, 7],
        false,
        vec![4_294_967_291, 4_294_967_290, 2],
    );
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        false,
        vec![4_294_967_294, 4_294_967_295, 1],
    );
    test(
        &[102, 101, 100],
        &[100, 101, 102],
        true,
        vec![2, 0, 4_294_967_294],
    );
    test(
        &[0, 0, 0],
        &[1],
        true,
        vec![4_294_967_295, 4_294_967_295, 4_294_967_295],
    );
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
    test(&[2], &[3], true, vec![4_294_967_295]);
    test(&[1, 2, 3], &[1, 1, 1], false, vec![0, 1, 2]);
    test(
        &[1, 1, 1],
        &[1, 2, 3],
        true,
        vec![0, 4_294_967_295, 4_294_967_293],
    );
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        false,
        vec![4_294_967_294, 4_294_967_295, 1],
    );
    test(
        &[102, 101, 100],
        &[100, 101, 102],
        true,
        vec![2, 0, 4_294_967_294],
    );
    test(
        &[0, 0, 0],
        &[1, 0, 0],
        true,
        vec![4_294_967_295, 4_294_967_295, 4_294_967_295],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_same_length_in_place_right_fail_1() {
    limbs_sub_same_length_in_place_right(&[6, 7], &mut vec![1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_sub_same_length_in_place_right_fail_2() {
    limbs_sub_same_length_in_place_right(&[1, 2], &mut vec![6, 7, 8]);
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
    test(
        &[123, 0],
        &[789, 123],
        1,
        true,
        &[4_294_966_630, 4_294_967_295],
    );
    test(&[123, 456], &[789, 123], 0, false, &[123, 456]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_slice_sub_in_place_right_fail_1() {
    limbs_slice_sub_in_place_right(&[6, 7], &mut vec![1, 2, 3], 1);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_slice_sub_in_place_right_fail_2() {
    limbs_slice_sub_in_place_right(&[6, 7], &mut vec![1, 2], 3);
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
    test(&[2], &[3], true, vec![4_294_967_295]);
    test(&[1, 2, 3], &[1, 1, 1], false, vec![0, 1, 2]);
    test(
        &[1, 1, 1],
        &[1, 2, 3],
        true,
        vec![0, 4_294_967_295, 4_294_967_293],
    );
    test(
        &[1, 2, 3],
        &[6, 7],
        false,
        vec![4_294_967_291, 4_294_967_290, 2],
    );
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        false,
        vec![4_294_967_294, 4_294_967_295, 1],
    );
    test(
        &[102, 101, 100],
        &[100, 101, 102],
        true,
        vec![2, 0, 4_294_967_294],
    );
    test(
        &[0, 0, 0],
        &[1],
        true,
        vec![4_294_967_295, 4_294_967_295, 4_294_967_295],
    );
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
    test(&[2], &[3], false, &[0, 0], true, vec![4_294_967_295, 0]);
    test(&[3], &[3], true, &[0, 0], true, vec![4_294_967_295, 0]);
    test(&[2], &[3], true, &[0, 0], true, vec![4_294_967_294, 0]);
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
        vec![0, 4_294_967_295, 4_294_967_293, 5],
    );
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        false,
        &[10, 10, 10, 10],
        false,
        vec![4_294_967_294, 4_294_967_295, 1, 10],
    );
    test(
        &[102, 101, 100],
        &[100, 101, 102],
        false,
        &[10, 10, 10, 10],
        true,
        vec![2, 0, 4_294_967_294, 10],
    );
    test(
        &[0, 0, 0],
        &[1, 0, 0],
        false,
        &[10, 10, 10, 10],
        true,
        vec![4_294_967_295, 4_294_967_295, 4_294_967_295, 10],
    );
    test(
        &[0, 0, 0],
        &[1, 0, 0],
        true,
        &[10, 10, 10, 10],
        true,
        vec![4_294_967_294, 4_294_967_295, 4_294_967_295, 10],
    );
    test(
        &[0, 0, 0],
        &[0, 0, 0],
        true,
        &[10, 10, 10, 10],
        true,
        vec![4_294_967_295, 4_294_967_295, 4_294_967_295, 10],
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
    test(&[2], &[3], false, true, vec![4_294_967_295]);
    test(&[2], &[2], true, true, vec![4_294_967_295]);
    test(&[2], &[3], true, true, vec![4_294_967_294]);
    test(&[1, 2, 3], &[1, 1, 1], false, false, vec![0, 1, 2]);
    test(
        &[1, 1, 1],
        &[1, 2, 3],
        false,
        true,
        vec![0, 4_294_967_295, 4_294_967_293],
    );
    test(
        &[100, 101, 102],
        &[102, 101, 100],
        false,
        false,
        vec![4_294_967_294, 4_294_967_295, 1],
    );
    test(
        &[102, 101, 100],
        &[100, 101, 102],
        false,
        true,
        vec![2, 0, 4_294_967_294],
    );
    test(
        &[0, 0, 0],
        &[1, 0, 0],
        false,
        true,
        vec![4_294_967_295, 4_294_967_295, 4_294_967_295],
    );
    test(
        &[0, 0, 0],
        &[1, 0, 0],
        true,
        true,
        vec![4_294_967_294, 4_294_967_295, 4_294_967_295],
    );
    test(
        &[0, 0, 0],
        &[0, 0, 0],
        true,
        true,
        vec![4_294_967_295, 4_294_967_295, 4_294_967_295],
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
        &[4_294_967_295, 4_294_967_294, 4_294_967_294, 4],
    );
    test(
        &[1, 2, 3, 4],
        2,
        true,
        &[4_294_967_294, 4_294_967_293, 3, 4],
    );
    test(&[1, 2, 3, 4], 3, true, &[4_294_967_293, 2, 3, 4]);
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
        &[4294967295, 4294967295, 0, 2],
    );
    test(&[1, 2, 3, 4], &[], false, &[1, 2, 3, 4]);
    test(&[1, 2, 3, 4], &[4], false, &[0, 2, 3, 4]);
    test(
        &[1, 2, 3, 4],
        &[4, 4],
        true,
        &[4294967295, 4294967295, 3, 4],
    );
    test(
        &[1, 2, 3, 4],
        &[4, 4, 4],
        true,
        &[4294967294, 4294967294, 4294967295, 4],
    );
    test(
        &[1, 2, 3, 4],
        &[4, 4, 4, 4],
        true,
        &[4294967293, 4294967293, 4294967294, 4294967295],
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
#[allow(unused_must_use)]
fn sub_fail_1() {
    Natural::from(123u32) - Natural::from(456u32);
}

#[test]
#[should_panic]
#[allow(unused_must_use)]
fn sub_fail_2() {
    Natural::from(123u32) - &Natural::from(456u32);
}

#[test]
#[should_panic]
#[allow(unused_must_use)]
fn sub_fail_3() {
    &Natural::from(123u32) - Natural::from(456u32);
}

#[test]
#[should_panic]
#[allow(unused_must_use)]
fn sub_fail_4() {
    &Natural::from(123u32) - &Natural::from(456u32);
}

#[test]
fn limbs_sub_limb_properties() {
    test_properties(pairs_of_unsigned_vec_and_unsigned, |&(ref limbs, limb)| {
        let (result_limbs, borrow) = limbs_sub_limb(limbs, limb);
        if borrow {
            if limbs.is_empty() {
                assert_ne!(limb, 0);
                assert!(result_limbs.is_empty());
            } else {
                let mut result_limbs = result_limbs;
                result_limbs.push(Limb::MAX);
                assert_eq!(
                    Integer::from_owned_twos_complement_limbs_asc(result_limbs),
                    Integer::from(Natural::from_limbs_asc(limbs)) - Integer::from(limb)
                );
            }
        } else {
            assert_eq!(
                Natural::from_owned_limbs_asc(result_limbs),
                Natural::from_limbs_asc(limbs) - Natural::from(limb)
            );
        }
    });
}

#[test]
fn limbs_sub_limb_to_out_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_1,
        |&(ref out, ref in_limbs, limb)| {
            let mut out = out.to_vec();
            let old_out = out.clone();
            if limbs_sub_limb_to_out(&mut out, in_limbs, limb) {
                let n = Integer::from(Natural::from_limbs_asc(in_limbs)) - Integer::from(limb);
                let len = in_limbs.len();
                let mut limbs = n.into_twos_complement_limbs_asc();
                limbs.resize(len, Limb::MAX);
                assert_eq!(limbs, &out[..len]);
                assert_eq!(&out[len..], &old_out[len..]);
            } else {
                let n = Natural::from_limbs_asc(in_limbs) - Natural::from(limb);
                let len = in_limbs.len();
                let mut limbs = n.into_limbs_asc();
                limbs.resize(len, 0);
                assert_eq!(limbs, &out[..len]);
                assert_eq!(&out[len..], &old_out[len..]);
            }
        },
    );
}

#[test]
fn limbs_sub_limb_in_place_properties() {
    test_properties(pairs_of_unsigned_vec_and_unsigned, |&(ref limbs, limb)| {
        let mut limbs = limbs.to_vec();
        let old_limbs = limbs.clone();
        if limbs_sub_limb_in_place(&mut limbs, limb) {
            let n = Integer::from(Natural::from_limbs_asc(&old_limbs)) - Integer::from(limb);
            let mut expected_limbs = n.into_twos_complement_limbs_asc();
            expected_limbs.resize(limbs.len(), Limb::MAX);
            assert_eq!(limbs, expected_limbs);
        } else {
            let n = Natural::from_limbs_asc(&old_limbs) - Natural::from(limb);
            let mut expected_limbs = n.into_limbs_asc();
            expected_limbs.resize(limbs.len(), 0);
            assert_eq!(limbs, expected_limbs);
        }
    });
}

#[test]
fn limbs_sub_properties() {
    test_properties(pairs_of_unsigned_vec_var_3, |&(ref xs, ref ys)| {
        let (limbs, borrow) = limbs_sub(xs, ys);
        let len = limbs.len();
        let n = Natural::from_owned_limbs_asc(limbs);
        if borrow {
            assert_eq!(
                n,
                Integer::from(Natural::from_limbs_asc(xs))
                    - Integer::from(Natural::from_limbs_asc(ys))
                    + (Integer::ONE << (usize::wrapping_from(Limb::WIDTH) * len))
            );
        } else {
            assert_eq!(n, Natural::from_limbs_asc(xs) - Natural::from_limbs_asc(ys));
        }
    });
}

fn limbs_sub_to_out_helper(
    f: &mut dyn FnMut(&mut [Limb], &[Limb], &[Limb]) -> bool,
    out: &Vec<Limb>,
    xs: &Vec<Limb>,
    ys: &Vec<Limb>,
) {
    let mut out = out.to_vec();
    let old_out = out.clone();
    let len = xs.len();
    let mut limbs = if f(&mut out, xs, ys) {
        let n = Integer::from(Natural::from_limbs_asc(xs))
            - Integer::from(Natural::from_limbs_asc(ys))
            + (Integer::ONE << (usize::wrapping_from(Limb::WIDTH) * len));
        Natural::checked_from(n).unwrap().into_limbs_asc()
    } else {
        let n = Natural::from_limbs_asc(xs) - Natural::from_limbs_asc(ys);
        n.into_limbs_asc()
    };
    limbs.resize(len, 0);
    assert_eq!(limbs, &out[..len]);
    assert_eq!(&out[len..], &old_out[len..]);
}

#[test]
fn limbs_sub_same_length_to_out_properties() {
    test_properties(
        triples_of_unsigned_vec_var_3,
        |&(ref out, ref xs, ref ys)| {
            limbs_sub_to_out_helper(&mut limbs_sub_same_length_to_out, out, xs, ys);
        },
    );
}

#[test]
fn limbs_sub_to_out_properties() {
    test_properties(
        triples_of_unsigned_vec_var_9,
        |&(ref out, ref xs, ref ys)| {
            limbs_sub_to_out_helper(&mut limbs_sub_to_out, out, xs, ys);
        },
    );
}

fn limbs_sub_in_place_left_helper(
    f: &mut dyn FnMut(&mut [Limb], &[Limb]) -> bool,
    xs: &Vec<Limb>,
    ys: &Vec<Limb>,
) {
    let mut xs = xs.to_vec();
    let xs_old = xs.clone();
    let len = xs.len();
    let borrow = f(&mut xs, ys);
    let n = Natural::from_owned_limbs_asc(xs);
    if borrow {
        assert_eq!(
            n,
            Integer::from(Natural::from_owned_limbs_asc(xs_old))
                - Integer::from(Natural::from_limbs_asc(ys))
                + (Integer::ONE << (usize::wrapping_from(Limb::WIDTH) * len))
        );
    } else {
        assert_eq!(
            n,
            Natural::from_owned_limbs_asc(xs_old) - Natural::from_limbs_asc(ys)
        );
    }
}

#[test]
fn limbs_sub_same_length_in_place_left_properties() {
    test_properties(pairs_of_unsigned_vec_var_1, |&(ref xs, ref ys)| {
        limbs_sub_in_place_left_helper(&mut limbs_sub_same_length_in_place_left, xs, ys);
    });
}

#[test]
fn limbs_sub_in_place_left_properties() {
    test_properties(pairs_of_unsigned_vec_var_3, |&(ref xs, ref ys)| {
        limbs_sub_in_place_left_helper(&mut limbs_sub_in_place_left, xs, ys);
    });
}

#[test]
fn limbs_slice_sub_in_place_right_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_and_small_usize_var_1,
        |&(ref xs, ref ys, len)| {
            let ys_old = ys;
            let mut ys = ys_old.clone();
            let borrow = limbs_slice_sub_in_place_right(xs, &mut ys, len);
            let xs_len = xs.len();
            let x = Natural::from_limbs_asc(xs);
            let y = Natural::from_limbs_asc(&ys_old[..len]);
            let n = Natural::from_limbs_asc(&ys[..xs_len]);
            if borrow {
                assert_eq!(
                    n,
                    Integer::from(x) - Integer::from(y)
                        + (Integer::ONE << (usize::wrapping_from(Limb::WIDTH) * xs_len))
                );
            } else {
                assert_eq!(n, x - y);
            }
        },
    );

    test_properties(pairs_of_unsigned_vec_var_1, |&(ref xs, ref ys)| {
        let ys_old = ys;
        let mut ys = ys_old.clone();
        assert!(!limbs_slice_sub_in_place_right(xs, &mut ys, 0));
        assert_eq!(*xs, ys);
    });
}

macro_rules! limbs_vec_sub_in_place_right_helper {
    ($f:ident, $xs:ident, $ys:ident) => {
        |&(ref $xs, ref $ys)| {
            let mut ys = $ys.to_vec();
            let ys_old = $ys.clone();
            let borrow = $f($xs, &mut ys);
            let n = Natural::from_limbs_asc(&ys);
            if borrow {
                assert_eq!(
                    n,
                    Integer::from(Natural::from_limbs_asc($xs))
                        - Integer::from(Natural::from_owned_limbs_asc(ys_old))
                        + (Integer::ONE << (usize::wrapping_from(Limb::WIDTH) * $xs.len()))
                );
            } else {
                assert_eq!(
                    n,
                    Natural::from_limbs_asc($xs) - Natural::from_owned_limbs_asc(ys_old)
                );
            }
        }
    };
}

#[test]
fn limbs_sub_same_length_in_place_right_properties() {
    test_properties(
        pairs_of_unsigned_vec_var_1,
        limbs_vec_sub_in_place_right_helper!(limbs_sub_same_length_in_place_right, xs, ys),
    );
}

#[test]
fn limbs_vec_sub_in_place_right_properties() {
    test_properties(
        pairs_of_unsigned_vec_var_3,
        limbs_vec_sub_in_place_right_helper!(limbs_vec_sub_in_place_right, xs, ys),
    );
}

#[test]
fn limbs_sub_same_length_with_borrow_in_to_out_properties() {
    test_properties(
        quadruples_of_three_unsigned_vecs_and_bool_var_1,
        |&(ref out, ref xs, ref ys, borrow_in)| {
            let mut out = out.to_vec();
            let old_out = out.clone();
            let len = xs.len();
            let limbs = if _limbs_sub_same_length_with_borrow_in_to_out(&mut out, xs, ys, borrow_in)
            {
                let mut n = Integer::from(Natural::from_limbs_asc(xs))
                    - Integer::from(Natural::from_limbs_asc(ys))
                    + (Integer::ONE << (usize::wrapping_from(Limb::WIDTH) * len));
                if borrow_in {
                    n.decrement();
                }
                let mut limbs = Natural::checked_from(n).unwrap().into_limbs_asc();
                limbs.resize(len, Limb::MAX);
                limbs
            } else {
                let mut n = Natural::from_limbs_asc(xs) - Natural::from_limbs_asc(ys);
                if borrow_in {
                    n.decrement();
                }
                let mut limbs = n.into_limbs_asc();
                limbs.resize(len, 0);
                limbs
            };
            assert_eq!(limbs, &out[..len]);
            assert_eq!(&out[len..], &old_out[len..]);
        },
    );
}

#[test]
fn limbs_sub_same_length_with_borrow_in_in_place_left_properties() {
    test_properties(
        triples_of_two_unsigned_vecs_and_bool_var_1,
        |&(ref xs, ref ys, borrow_in)| {
            let mut xs = xs.to_vec();
            let xs_old = xs.clone();
            let len = xs.len();
            let borrow =
                _limbs_sub_same_length_with_borrow_in_in_place_left(&mut xs, ys, borrow_in);
            let n = Natural::from_owned_limbs_asc(xs);
            let mut expected_result = if borrow {
                Natural::checked_from(
                    Integer::from(Natural::from_owned_limbs_asc(xs_old))
                        - Integer::from(Natural::from_limbs_asc(ys))
                        + (Integer::ONE << (usize::wrapping_from(Limb::WIDTH) * len)),
                )
                .unwrap()
            } else {
                Natural::from_owned_limbs_asc(xs_old) - Natural::from_limbs_asc(ys)
            };
            if borrow_in {
                expected_result.decrement();
            }
            assert_eq!(n, expected_result);
        },
    );
}

#[test]
fn limbs_sub_same_length_in_place_with_overlap_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_usize_var_1,
        |&(ref xs, right_start)| {
            let xs_old = xs;
            let mut xs = xs_old.clone();
            let borrow = limbs_sub_same_length_in_place_with_overlap(&mut xs, right_start);
            let len = xs.len() - right_start;
            let x = Natural::from_limbs_asc(&xs_old[..len]);
            let y = Natural::from_limbs_asc(&xs_old[right_start..]);
            let n = Natural::from_limbs_asc(&xs[..len]);
            if borrow {
                assert_eq!(
                    n,
                    Integer::from(x) - Integer::from(y)
                        + (Integer::ONE << (usize::wrapping_from(Limb::WIDTH) * len))
                );
            } else {
                assert_eq!(n, x - y);
            }
            assert_eq!(&xs[len..], &xs_old[len..]);

            let mut xs_alt = xs_old.clone();
            assert_eq!(
                limbs_sub_same_length_in_place_with_overlap_naive(&mut xs_alt, right_start),
                borrow
            );
            assert_eq!(xs_alt, xs);
        },
    );

    test_properties(vecs_of_unsigned, |ref xs| {
        let xs_old = xs;
        let mut xs = xs_old.to_vec();
        assert!(!limbs_sub_same_length_in_place_with_overlap(&mut xs, 0));
        assert!(limbs_test_zero(&xs));

        let mut xs = xs_old.to_vec();
        assert!(!limbs_sub_same_length_in_place_with_overlap(
            &mut xs,
            xs_old.len(),
        ));
        assert_eq!(xs, **xs_old);
    });
}

#[test]
fn limbs_sub_same_length_to_out_with_overlap_properties() {
    test_properties(pairs_of_unsigned_vec_var_3, |&(ref xs, ref ys)| {
        let xs_old = xs;
        let mut xs = xs_old.clone();
        let borrow = limbs_sub_same_length_to_out_with_overlap(&mut xs, ys);
        let len = ys.len();
        let x = Natural::from_limbs_asc(&xs_old[xs.len() - len..]);
        let y = Natural::from_limbs_asc(ys);
        let n = Natural::from_limbs_asc(&xs[..len]);
        if borrow {
            assert_eq!(
                n,
                Integer::from(x) - Integer::from(y)
                    + (Integer::ONE << (usize::wrapping_from(Limb::WIDTH) * len))
            );
        } else {
            assert_eq!(n, x - y);
        }
        if len <= xs.len() - len {
            assert_eq!(&xs[len..xs.len() - len], &xs_old[len..xs.len() - len]);
        }

        let mut xs_alt = xs_old.clone();
        assert_eq!(
            limbs_sub_same_length_to_out_with_overlap_naive(&mut xs_alt, ys),
            borrow
        );
        assert_eq!(xs_alt, xs);
    });

    test_properties(vecs_of_unsigned, |ref xs| {
        let xs_old = xs;
        let mut xs = xs_old.to_vec();
        assert!(!limbs_sub_same_length_to_out_with_overlap(&mut xs, xs_old));
        assert!(limbs_test_zero(&xs));

        let mut xs = xs_old.to_vec();
        assert!(!limbs_sub_same_length_to_out_with_overlap(&mut xs, &[]));
        assert_eq!(xs, **xs_old);
    });
}

#[test]
fn sub_properties() {
    test_properties(pairs_of_naturals_var_1, |&(ref x, ref y)| {
        let mut mut_x = x.clone();
        mut_x -= y.clone();
        assert!(mut_x.is_valid());
        let difference = mut_x;

        let mut mut_x = x.clone();
        mut_x -= y;
        assert!(mut_x.is_valid());
        let difference_alt = mut_x;
        assert_eq!(difference_alt, difference);

        let mut rug_x = natural_to_rug_integer(x);
        rug_x -= natural_to_rug_integer(y);
        assert_eq!(rug_integer_to_natural(&rug_x), difference);

        let difference_alt = x.clone() - y.clone();
        assert!(difference_alt.is_valid());
        assert_eq!(difference_alt, difference);

        let difference_alt = x.clone() - y;
        assert_eq!(difference_alt, difference);
        assert!(difference_alt.is_valid());

        let difference_alt = x - y.clone();
        assert_eq!(difference_alt, difference);
        assert!(difference_alt.is_valid());

        let difference_alt = x - y;
        assert_eq!(difference_alt, difference);
        assert!(difference_alt.is_valid());

        assert_eq!(
            biguint_to_natural(&(natural_to_biguint(x) - natural_to_biguint(y))),
            difference
        );
        assert_eq!(
            rug_integer_to_natural(&(natural_to_rug_integer(x) - natural_to_rug_integer(y))),
            difference
        );

        assert!(difference <= *x);
        assert_eq!(difference + y, *x);
    });

    test_properties(pairs_of_unsigneds_var_1::<Limb>, |&(x, y)| {
        assert_eq!(Natural::from(x - y), Natural::from(x) - Natural::from(y));
    });

    #[allow(unknown_lints, identity_op, eq_op)]
    test_properties(naturals, |x| {
        assert_eq!(x - Natural::ZERO, *x);
        assert_eq!(x - x, Natural::ZERO);
    });
}
