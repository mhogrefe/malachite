use std::cmp::max;
use std::str::FromStr;

use malachite_base::num::basic::traits::{One, Zero};
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
use malachite_nz::platform::{DoubleLimb, Limb};
use num::BigUint;
use rug;

use malachite_test::common::test_properties;
use malachite_test::common::{
    biguint_to_natural, natural_to_biguint, natural_to_rug_integer, rug_integer_to_natural,
};
use malachite_test::inputs::base::{
    pairs_of_nonempty_unsigned_vec_and_unsigned, pairs_of_unsigned_vec,
    pairs_of_unsigned_vec_and_unsigned, pairs_of_unsigned_vec_var_1, pairs_of_unsigned_vec_var_3,
    pairs_of_unsigneds, quadruples_of_three_unsigned_vecs_and_bool_var_1,
    triples_of_two_unsigned_vecs_and_bool_var_1,
    triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_1,
    triples_of_unsigned_vec_usize_and_unsigned_vec_var_1, triples_of_unsigned_vec_var_3,
    triples_of_unsigned_vec_var_4, triples_of_unsigned_vec_var_9,
};
use malachite_test::inputs::natural::{naturals, pairs_of_naturals, triples_of_naturals};

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_add_limb() {
    let test = |limbs: &[Limb], limb: Limb, out: &[Limb]| {
        assert_eq!(limbs_add_limb(limbs, limb), out);
    };
    test(&[], 0, &[]);
    test(&[], 5, &[5]);
    test(&[6, 7], 2, &[8, 7]);
    test(&[100, 101, 102], 10, &[110, 101, 102]);
    test(&[123, 456], 789, &[912, 456]);
    test(&[0xffff_ffff, 5], 2, &[1, 6]);
    test(&[0xffff_ffff], 2, &[1, 1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_add_limb_to_out() {
    let test =
        |out_before: &[Limb], limbs_in: &[Limb], limb: Limb, carry: bool, out_after: &[Limb]| {
            let mut out = out_before.to_vec();
            assert_eq!(limbs_add_limb_to_out(&mut out, limbs_in, limb), carry);
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
    test(
        &[10, 10, 10, 10],
        &[0xffff_ffff, 5],
        2,
        false,
        &[1, 6, 10, 10],
    );
    test(&[10, 10, 10, 10], &[0xffff_ffff], 2, true, &[1, 10, 10, 10]);
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
    let test = |limbs: &[Limb], limb: Limb, carry: bool, out: &[Limb]| {
        let mut limbs = limbs.to_vec();
        assert_eq!(limbs_slice_add_limb_in_place(&mut limbs, limb), carry);
        assert_eq!(limbs, out);
    };
    test(&[], 0, false, &[]);
    test(&[], 5, true, &[]);
    test(&[6, 7], 2, false, &[8, 7]);
    test(&[100, 101, 102], 10, false, &[110, 101, 102]);
    test(&[123, 456], 789, false, &[912, 456]);
    test(&[0xffff_ffff, 5], 2, false, &[1, 6]);
    test(&[0xffff_ffff], 2, true, &[1]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_vec_add_limb_in_place() {
    let test = |limbs: &[Limb], limb: Limb, out: &[Limb]| {
        let mut limbs = limbs.to_vec();
        limbs_vec_add_limb_in_place(&mut limbs, limb);
        assert_eq!(limbs, out);
    };
    test(&[6, 7], 2, &[8, 7]);
    test(&[100, 101, 102], 10, &[110, 101, 102]);
    test(&[123, 456], 789, &[912, 456]);
    test(&[0xffff_ffff, 5], 2, &[1, 6]);
    test(&[0xffff_ffff], 2, &[1, 1]);
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
    test(&[0xffff_ffff, 3], &[1], vec![0, 4]);
    test(&[0xffff_ffff], &[1], vec![0, 1]);
    test(&[1], &[0xffff_ffff], vec![0, 1]);
    test(&[0xffff_ffff], &[0xffff_ffff], vec![0xffff_fffe, 1]);
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
    test(&[0xffff_ffff, 3], &[1], vec![0, 4]);
    test(&[1], &[0xffff_ffff, 3], vec![0, 4]);
    test(&[0xffff_ffff], &[1], vec![0, 1]);
    test(&[1], &[0xffff_ffff], vec![0, 1]);
    test(&[0xffff_ffff], &[0xffff_ffff], vec![0xffff_fffe, 1]);
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
        &[0xffff_ffff, 3],
        &[1],
        &[10, 10, 10, 10],
        false,
        vec![0, 4, 10, 10],
    );
    test(
        &[0xffff_ffff],
        &[1],
        &[10, 10, 10, 10],
        true,
        vec![0, 10, 10, 10],
    );
    test(
        &[1],
        &[0xffff_ffff],
        &[10, 10, 10, 10],
        true,
        vec![0, 10, 10, 10],
    );
    test(
        &[0xffff_ffff],
        &[0xffff_ffff],
        &[10, 10, 10, 10],
        true,
        vec![0xffff_fffe, 10, 10, 10],
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
        &[0xffff_ffff],
        &[1],
        &[10, 10, 10, 10],
        true,
        vec![0, 10, 10, 10],
    );
    test(
        &[1],
        &[0xffff_ffff],
        &[10, 10, 10, 10],
        true,
        vec![0, 10, 10, 10],
    );
    test(
        &[0xffff_ffff],
        &[0xffff_ffff],
        &[10, 10, 10, 10],
        true,
        vec![0xffff_fffe, 10, 10, 10],
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
        &[0xffff_ffff, 3],
        &[1],
        &[10, 10, 10, 10],
        false,
        vec![0, 4, 10, 10],
    );
    test(
        &[1],
        &[0xffff_ffff, 3],
        &[10, 10, 10, 10],
        false,
        vec![0, 4, 10, 10],
    );
    test(
        &[0xffff_ffff],
        &[1],
        &[10, 10, 10, 10],
        true,
        vec![0, 10, 10, 10],
    );
    test(
        &[1],
        &[0xffff_ffff],
        &[10, 10, 10, 10],
        true,
        vec![0, 10, 10, 10],
    );
    test(
        &[0xffff_ffff],
        &[0xffff_ffff],
        &[10, 10, 10, 10],
        true,
        vec![0xffff_fffe, 10, 10, 10],
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
    test(&[1, 1, 3], 1, &[0xffff_ffff, 5], false, vec![0, 6, 3]);
    test(&[1, 1, 3], 1, &[0xffff_ffff], true, vec![0, 1, 3]);
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
    test(&[0xffff_ffff], &[1], true, vec![0]);
    test(&[1], &[0xffff_ffff], true, vec![0]);
    test(&[0xffff_ffff], &[0xffff_ffff], true, vec![0xffff_fffe]);
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
    test(&[0xffff_ffff, 3], &[1], false, vec![0, 4]);
    test(&[0xffff_ffff], &[1], true, vec![0]);
    test(&[1], &[0xffff_ffff], true, vec![0]);
    test(&[0xffff_ffff], &[0xffff_ffff], true, vec![0xffff_fffe]);
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
    test(&[0xffff_ffff, 3], &[1], (false, false), vec![0, 4], vec![1]);
    test(&[1], &[0xffff_ffff, 3], (true, false), vec![1], vec![0, 4]);
    test(&[0xffff_ffff], &[1], (false, true), vec![0], vec![1]);
    test(
        &[1],
        &[0xffff_ffff],
        (false, true),
        vec![0],
        vec![0xffff_ffff],
    );
    test(
        &[0xffff_ffff],
        &[0xffff_ffff],
        (false, true),
        vec![0xffff_fffe],
        vec![0xffff_ffff],
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
    test(&[0xffff_ffff, 3], &[1], false, vec![0, 4], vec![1]);
    test(&[1], &[0xffff_ffff, 3], true, vec![1], vec![0, 4]);
    test(&[0xffff_ffff], &[1], false, vec![0, 1], vec![1]);
    test(&[1], &[0xffff_ffff], false, vec![0, 1], vec![0xffff_ffff]);
    test(
        &[0xffff_ffff],
        &[0xffff_ffff],
        false,
        vec![0xffff_fffe, 1],
        vec![0xffff_ffff],
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
        &[0xffff_ffff],
        &[1],
        false,
        &[10, 10, 10, 10],
        true,
        vec![0, 10, 10, 10],
    );
    test(
        &[0xffff_ffff],
        &[1],
        true,
        &[10, 10, 10, 10],
        true,
        vec![1, 10, 10, 10],
    );
    test(
        &[0xffff_fffe],
        &[1],
        true,
        &[10, 10, 10, 10],
        true,
        vec![0, 10, 10, 10],
    );
    test(
        &[1],
        &[0xffff_ffff],
        false,
        &[10, 10, 10, 10],
        true,
        vec![0, 10, 10, 10],
    );
    test(
        &[0xffff_ffff],
        &[0xffff_ffff],
        false,
        &[10, 10, 10, 10],
        true,
        vec![0xffff_fffe, 10, 10, 10],
    );
    test(
        &[0xffff_ffff],
        &[0xffff_ffff],
        true,
        &[10, 10, 10, 10],
        true,
        vec![0xffff_ffff, 10, 10, 10],
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
    test(&[0xffff_ffff], &[1], false, true, vec![0]);
    test(&[0xffff_ffff], &[1], true, true, vec![1]);
    test(&[0xffff_fffe], &[1], true, true, vec![0]);
    test(&[1], &[0xffff_ffff], false, true, vec![0]);
    test(
        &[0xffff_ffff],
        &[0xffff_ffff],
        false,
        true,
        vec![0xffff_fffe],
    );
    test(
        &[0xffff_ffff],
        &[0xffff_ffff],
        true,
        true,
        vec![0xffff_ffff],
    );
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
    let test = |u, v, out| {
        let mut n = Natural::from_str(u).unwrap();
        n += Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = Natural::from_str(u).unwrap();
        n += &Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u).unwrap() + Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Natural::from_str(u).unwrap() + Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = Natural::from_str(u).unwrap() + &Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = &Natural::from_str(u).unwrap() + &Natural::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = BigUint::from_str(u).unwrap() + BigUint::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);

        let n = rug::Integer::from_str(u).unwrap() + rug::Integer::from_str(v).unwrap();
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
fn limbs_add_limb_properties() {
    test_properties(pairs_of_unsigned_vec_and_unsigned, |&(ref limbs, limb)| {
        assert_eq!(
            Natural::from_owned_limbs_asc(limbs_add_limb(limbs, limb)),
            Natural::from_limbs_asc(limbs) + Natural::from(limb)
        );
    });
}

#[test]
fn limbs_add_limb_to_out_properties() {
    test_properties(
        triples_of_unsigned_vec_unsigned_vec_and_unsigned_var_1,
        |&(ref out, ref in_limbs, limb)| {
            let mut out = out.to_vec();
            let old_out = out.clone();
            let carry = limbs_add_limb_to_out(&mut out, in_limbs, limb);
            let n = Natural::from_limbs_asc(in_limbs) + Natural::from(limb);
            let len = in_limbs.len();
            let mut limbs = n.into_limbs_asc();
            assert_eq!(carry, limbs.len() == len + 1);
            limbs.resize(len, 0);
            assert_eq!(limbs, &out[..len]);
            assert_eq!(&out[len..], &old_out[len..]);
        },
    );
}

#[test]
fn limbs_slice_add_limb_in_place_properties() {
    test_properties(pairs_of_unsigned_vec_and_unsigned, |&(ref limbs, limb)| {
        let mut limbs = limbs.to_vec();
        let old_limbs = limbs.clone();
        let carry = limbs_slice_add_limb_in_place(&mut limbs, limb);
        let n = Natural::from_limbs_asc(&old_limbs) + Natural::from(limb);
        let mut expected_limbs = n.into_limbs_asc();
        assert_eq!(carry, expected_limbs.len() == limbs.len() + 1);
        expected_limbs.resize(limbs.len(), 0);
        assert_eq!(limbs, expected_limbs);
    });
}

#[test]
fn limbs_vec_add_limb_in_place_properties() {
    test_properties(
        pairs_of_nonempty_unsigned_vec_and_unsigned,
        |&(ref limbs, limb)| {
            let mut limbs = limbs.to_vec();
            let old_limbs = limbs.clone();
            limbs_vec_add_limb_in_place(&mut limbs, limb);
            let n = Natural::from_limbs_asc(&old_limbs) + Natural::from(limb);
            assert_eq!(Natural::from_owned_limbs_asc(limbs), n);
        },
    );
}

fn limbs_add_helper(f: &dyn Fn(&[Limb], &[Limb]) -> Vec<Limb>, xs: &Vec<Limb>, ys: &Vec<Limb>) {
    assert_eq!(
        Natural::from_owned_limbs_asc(f(xs, ys)),
        Natural::from_limbs_asc(xs) + Natural::from_limbs_asc(ys)
    );
}

#[test]
fn limbs_add_greater_properties() {
    test_properties(pairs_of_unsigned_vec_var_3, |&(ref xs, ref ys)| {
        limbs_add_helper(&limbs_add_greater, xs, ys);
    });
}

#[test]
fn limbs_add_properties() {
    test_properties(pairs_of_unsigned_vec, |&(ref xs, ref ys)| {
        limbs_add_helper(&limbs_add, xs, ys);
    });
}

fn limbs_add_to_out_helper(
    f: &mut dyn FnMut(&mut [Limb], &[Limb], &[Limb]) -> bool,
    out_len: &dyn Fn(usize, usize) -> usize,
    out: &Vec<Limb>,
    xs: &Vec<Limb>,
    ys: &Vec<Limb>,
) {
    let mut out = out.to_vec();
    let old_out = out.clone();
    let carry = f(&mut out, xs, ys);
    let n = Natural::from_limbs_asc(xs) + Natural::from_limbs_asc(ys);
    let len = out_len(xs.len(), ys.len());
    let mut limbs = n.into_limbs_asc();
    assert_eq!(carry, limbs.len() == len + 1);
    limbs.resize(len, 0);
    assert_eq!(limbs, &out[..len]);
    assert_eq!(&out[len..], &old_out[len..]);
}

#[test]
fn limbs_add_greater_to_out_properties() {
    test_properties(
        triples_of_unsigned_vec_var_9,
        |&(ref out, ref xs, ref ys)| {
            limbs_add_to_out_helper(
                &mut limbs_add_greater_to_out,
                &|xs_len, _| xs_len,
                out,
                xs,
                ys,
            );
        },
    );
}

#[test]
fn limbs_add_same_length_to_out_properties() {
    test_properties(
        triples_of_unsigned_vec_var_3,
        |&(ref out, ref xs, ref ys)| {
            limbs_add_to_out_helper(
                &mut limbs_add_same_length_to_out,
                &|xs_len, _| xs_len,
                out,
                xs,
                ys,
            );
        },
    );
}

#[test]
fn limbs_add_to_out_properties() {
    test_properties(
        triples_of_unsigned_vec_var_4,
        |&(ref out, ref xs, ref ys)| {
            limbs_add_to_out_helper(&mut limbs_add_to_out, &max, out, xs, ys);
        },
    );
}

#[test]
fn limbs_add_to_out_aliased_properties() {
    test_properties(
        triples_of_unsigned_vec_usize_and_unsigned_vec_var_1,
        |&(ref xs, in_size, ref ys)| {
            let mut xs = xs.to_vec();
            let xs_old = xs.clone();
            let carry = _limbs_add_to_out_aliased(&mut xs, in_size, ys);
            let n = Natural::from_limbs_asc(&xs_old[..in_size]) + Natural::from_limbs_asc(ys);
            let mut limbs = n.into_limbs_asc();
            let ys_len = ys.len();
            if limbs.len() < ys_len {
                limbs.resize(ys_len, 0);
            }
            assert_eq!(
                Natural::from_limbs_asc(&xs[..ys_len]),
                Natural::from_limbs_asc(&limbs[..ys_len]),
            );
            if carry {
                assert_eq!(limbs.len(), ys_len + 1);
                assert_eq!(*limbs.last().unwrap(), 1);
            } else {
                assert_eq!(limbs.len(), ys_len);
            }
            assert_eq!(&xs[ys_len..], &xs_old[ys_len..]);
        },
    );
}

fn limbs_slice_add_in_place_left_helper(
    f: &mut dyn FnMut(&mut [Limb], &[Limb]) -> bool,
    xs: &Vec<Limb>,
    ys: &Vec<Limb>,
) {
    let mut xs = xs.to_vec();
    let xs_old = xs.clone();
    let carry = f(&mut xs, ys);
    let n = Natural::from_owned_limbs_asc(xs_old) + Natural::from_limbs_asc(ys);
    let len = xs.len();
    let mut limbs = n.into_limbs_asc();
    assert_eq!(carry, limbs.len() == len + 1);
    limbs.resize(len, 0);
    assert_eq!(limbs, xs);
}

#[test]
fn limbs_slice_add_same_length_in_place_left_properties() {
    test_properties(pairs_of_unsigned_vec_var_1, |&(ref xs, ref ys)| {
        limbs_slice_add_in_place_left_helper(
            &mut limbs_slice_add_same_length_in_place_left,
            xs,
            ys,
        );
    });
}

#[test]
fn limbs_slice_add_greater_in_place_left_properties() {
    test_properties(pairs_of_unsigned_vec_var_3, |&(ref xs, ref ys)| {
        limbs_slice_add_in_place_left_helper(&mut limbs_slice_add_greater_in_place_left, xs, ys);
    });
}

#[test]
fn limbs_vec_add_in_place_left_properties() {
    test_properties(pairs_of_unsigned_vec, |&(ref xs, ref ys)| {
        let mut xs = xs.to_vec();
        let xs_old = xs.clone();
        limbs_vec_add_in_place_left(&mut xs, ys);
        assert_eq!(
            Natural::from_owned_limbs_asc(xs),
            Natural::from_owned_limbs_asc(xs_old) + Natural::from_limbs_asc(ys)
        );
    });
}

#[test]
fn limbs_slice_add_in_place_either_properties() {
    test_properties(pairs_of_unsigned_vec, |&(ref xs, ref ys)| {
        let mut xs = xs.to_vec();
        let mut ys = ys.to_vec();
        let xs_old = xs.clone();
        let ys_old = ys.clone();
        let (right, b) = limbs_slice_add_in_place_either(&mut xs, &mut ys);
        let len = max(xs_old.len(), ys_old.len());
        let result = Natural::from_limbs_asc(&xs_old) + Natural::from_limbs_asc(&ys_old);
        let mut expected_limbs = result.to_limbs_asc();
        expected_limbs.resize(len, 0);
        assert_eq!(!b, Natural::from_limbs_asc(&expected_limbs) == result);
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
fn limbs_vec_add_in_place_either_properties() {
    test_properties(pairs_of_unsigned_vec, |&(ref xs, ref ys)| {
        let mut xs = xs.to_vec();
        let mut ys = ys.to_vec();
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
    test_properties(
        quadruples_of_three_unsigned_vecs_and_bool_var_1,
        |&(ref out, ref xs, ref ys, carry_in)| {
            let mut out = out.to_vec();
            let old_out = out.clone();
            let carry = _limbs_add_same_length_with_carry_in_to_out(&mut out, xs, ys, carry_in);
            let mut n = Natural::from_limbs_asc(xs) + Natural::from_limbs_asc(ys);
            if carry_in {
                n += Natural::ONE;
            }
            let len = xs.len();
            let mut limbs = n.into_limbs_asc();
            assert_eq!(carry, limbs.len() == len + 1);
            limbs.resize(len, 0);
            assert_eq!(limbs, &out[..len]);
            assert_eq!(&out[len..], &old_out[len..]);
        },
    );
}

#[test]
fn limbs_add_same_length_with_carry_in_in_place_left_properties() {
    test_properties(
        triples_of_two_unsigned_vecs_and_bool_var_1,
        |&(ref xs, ref ys, carry_in)| {
            let mut xs = xs.to_vec();
            let xs_old = xs.clone();
            let carry = _limbs_add_same_length_with_carry_in_in_place_left(&mut xs, ys, carry_in);
            let mut n = Natural::from_owned_limbs_asc(xs_old) + Natural::from_limbs_asc(ys);
            if carry_in {
                n += Natural::ONE;
            }
            let len = xs.len();
            let mut limbs = n.into_limbs_asc();
            assert_eq!(carry, limbs.len() == len + 1);
            limbs.resize(len, 0);
            assert_eq!(limbs, xs);
        },
    );
}

#[test]
fn add_properties() {
    test_properties(pairs_of_naturals, |&(ref x, ref y)| {
        let sum_val_val = x.clone() + y.clone();
        let sum_val_ref = x.clone() + y;
        let sum_ref_val = x + y.clone();
        let sum = x + y;
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
        mut_x += y;
        assert_eq!(mut_x, sum);
        assert!(mut_x.is_valid());

        let mut mut_x = natural_to_rug_integer(x);
        mut_x += natural_to_rug_integer(y);
        assert_eq!(rug_integer_to_natural(&mut_x), sum);

        assert_eq!(
            biguint_to_natural(&(natural_to_biguint(x) + natural_to_biguint(y))),
            sum
        );
        assert_eq!(
            rug_integer_to_natural(&(natural_to_rug_integer(x) + natural_to_rug_integer(y))),
            sum
        );
        assert_eq!(y + x, sum);
        assert_eq!(&sum - x, *y);
        assert_eq!(&sum - y, *x);

        assert!(sum >= *x);
        assert!(sum >= *y);
    });

    test_properties(pairs_of_unsigneds::<Limb>, |&(x, y)| {
        assert_eq!(
            Natural::from(DoubleLimb::from(x) + DoubleLimb::from(y)),
            Natural::from(x) + Natural::from(y)
        );
    });

    test_properties(naturals, |x| {
        assert_eq!(x + Natural::ZERO, *x);
        assert_eq!(Natural::ZERO + x, *x);
        assert_eq!(x + x, x << 1);
    });

    test_properties(triples_of_naturals, |&(ref x, ref y, ref z)| {
        assert_eq!((x + y) + z, x + (y + z));
    });
}
