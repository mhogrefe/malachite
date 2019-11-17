use malachite_base::limbs::limbs_move_left;

#[test]
fn test_limbs_move_left() {
    let test = |limbs_in: &[u32], amount, limbs_out: &[u32]| {
        let mut limbs = limbs_in.to_vec();
        limbs_move_left(&mut limbs, amount);
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
