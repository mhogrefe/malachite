use num::BigUint;
use rug;
use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
use malachite_nz::natural::arithmetic::add::{
    _limbs_add_same_length_with_carry_in_in_place_left,
    _limbs_add_same_length_with_carry_in_to_out, _limbs_add_to_out_aliased, limbs_add,
    limbs_add_greater, limbs_add_greater_to_out, limbs_add_limb, limbs_add_limb_to_out,
    limbs_add_same_length_to_out, limbs_add_to_out, limbs_slice_add_greater_in_place_left,
    limbs_slice_add_in_place_either, limbs_slice_add_limb_in_place,
    limbs_slice_add_same_length_in_place_left, limbs_vec_add_in_place_either,
    limbs_vec_add_in_place_left, limbs_vec_add_limb_in_place,
};
use malachite_nz::natural::Natural;
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::platform::Limb;

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
        assert_eq!(_limbs_add_to_out_aliased(&mut xs, in_size, ys), carry);
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
    _limbs_add_to_out_aliased(&mut out, 1, &[1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_add_to_out_aliased_fail_2() {
    let mut out = vec![6, 7, 8, 9];
    _limbs_add_to_out_aliased(&mut out, 4, &[1, 2, 3]);
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
            _limbs_add_same_length_with_carry_in_to_out(&mut out, xs, ys, carry_in),
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
    _limbs_add_same_length_with_carry_in_to_out(&mut out, &[6, 7], &[1, 2, 3], false);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_add_same_length_with_carry_in_to_out_fail_2() {
    let mut out = vec![10];
    _limbs_add_same_length_with_carry_in_to_out(&mut out, &[6, 7], &[1, 2], false);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_add_same_length_with_carry_in_in_place_left() {
    let test = |xs_before: &[Limb], ys, carry_in, carry, xs_after| {
        let mut xs = xs_before.to_vec();
        assert_eq!(
            _limbs_add_same_length_with_carry_in_in_place_left(&mut xs, ys, carry_in),
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
    _limbs_add_same_length_with_carry_in_in_place_left(&mut out, &[1, 2, 3], false);
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
