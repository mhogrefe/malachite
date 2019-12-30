extern crate malachite_base;

use malachite_base::limbs::{
    limbs_delete_left, limbs_leading_zero_limbs, limbs_move_left, limbs_pad_left, limbs_set_zero,
    limbs_test_zero, limbs_trailing_zero_limbs,
};

#[test]
fn test_limbs_test_zero() {
    let test = |limbs: &[u32], out| {
        assert_eq!(limbs_test_zero(limbs), out);
    };
    test(&[], true);
    test(&[0], true);
    test(&[0, 0, 0], true);
    test(&[123], false);
    test(&[123, 0], false);
    test(&[0, 123, 0, 0, 0], false);
}

#[test]
fn test_limbs_set_zero() {
    let test = |limbs: &[u32], out: &[u32]| {
        let mut mut_limbs = limbs.to_vec();
        limbs_set_zero(&mut mut_limbs);
        assert_eq!(mut_limbs, out);
    };
    test(&[], &[]);
    test(&[0], &[0]);
    test(&[0, 0, 0], &[0, 0, 0]);
    test(&[123], &[0]);
    test(&[123, 0], &[0, 0]);
    test(&[0, 123, 0, 0, 0], &[0, 0, 0, 0, 0]);
}

#[test]
fn test_limbs_pad_left() {
    let test = |limbs: &[u32], pad_size: usize, pad_limb: u32, out: &[u32]| {
        let mut mut_limbs = limbs.to_vec();
        limbs_pad_left(&mut mut_limbs, pad_size, pad_limb);
        assert_eq!(mut_limbs, out);
    };
    test(&[], 3, 6, &[6, 6, 6]);
    test(&[1, 2, 3], 0, 10, &[1, 2, 3]);
    test(&[1, 2, 3], 5, 10, &[10, 10, 10, 10, 10, 1, 2, 3]);
}

#[test]
fn test_limbs_delete_left() {
    let test = |limbs: &[u32], delete_size: usize, out: &[u32]| {
        let mut mut_limbs = limbs.to_vec();
        limbs_delete_left(&mut mut_limbs, delete_size);
        assert_eq!(mut_limbs, out);
    };
    test(&[], 0, &[]);
    test(&[1, 2, 3, 4, 5], 0, &[1, 2, 3, 4, 5]);
    test(&[1, 2, 3, 4, 5], 3, &[4, 5]);
    test(&[1, 2, 3, 4, 5], 5, &[]);
}

#[test]
#[should_panic]
fn limbs_delete_left_fail_1() {
    let mut limbs: Vec<u32> = Vec::new();
    limbs_delete_left(&mut limbs, 1);
}

#[test]
#[should_panic]
fn limbs_delete_left_fail_2() {
    let mut limbs: Vec<u32> = vec![1, 2, 3];
    limbs_delete_left(&mut limbs, 4);
}

fn limbs_move_left_naive<T: Copy>(limbs: &mut [u32], amount: usize) {
    let slice = limbs[amount..].to_vec();
    let limit = limbs.len() - amount;
    limbs[..limit].copy_from_slice(&slice);
}

#[test]
fn test_limbs_move_left() {
    let test = |limbs_in: &[u32], amount, limbs_out: &[u32]| {
        let mut limbs = limbs_in.to_vec();
        limbs_move_left(&mut limbs, amount);
        assert_eq!(limbs, limbs_out);

        let mut limbs = limbs_in.to_vec();
        limbs_move_left_naive::<u32>(&mut limbs, amount);
        assert_eq!(limbs, limbs_out);
    };
    test(&[], 0, &[]);
    test(&[1], 0, &[1]);
    test(&[1], 1, &[1]);
    test(&[1, 2, 3], 0, &[1, 2, 3]);
    test(&[1, 2, 3], 1, &[2, 3, 3]);
    test(&[1, 2, 3], 2, &[3, 2, 3]);
    test(&[1, 2, 3], 3, &[1, 2, 3]);
    test(&[1, 2, 3, 4, 5, 6], 2, &[3, 4, 5, 6, 5, 6])
}

#[test]
#[should_panic]
fn limbs_move_left_fail_1() {
    let limbs = &mut [];
    limbs_move_left::<u32>(limbs, 1);
}

#[test]
#[should_panic]
fn limbs_move_left_fail_2() {
    let limbs = &mut [1, 2, 3];
    limbs_move_left::<u32>(limbs, 4);
}

#[test]
fn test_limbs_leading_zero_limbs() {
    let test = |limbs: &[u32], out| {
        assert_eq!(limbs_leading_zero_limbs(limbs), out);
    };
    test(&[], 0);
    test(&[0], 1);
    test(&[0, 0, 0], 3);
    test(&[123], 0);
    test(&[123, 0], 0);
    test(&[0, 123, 0, 0, 0], 1);
    test(&[1, 2, 3], 0);
    test(&[0, 0, 0, 1, 2, 3], 3);
}

#[test]
fn test_limbs_trailing_zero_limbs() {
    let test = |limbs: &[u32], out| {
        assert_eq!(limbs_trailing_zero_limbs(limbs), out);
    };
    test(&[], 0);
    test(&[0], 1);
    test(&[0, 0, 0], 3);
    test(&[123], 0);
    test(&[123, 0], 1);
    test(&[0, 123, 0, 0, 0], 3);
    test(&[1, 2, 3], 0);
    test(&[1, 2, 3, 0, 0, 0], 3);
}
