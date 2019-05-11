use malachite_base::limbs::limbs_delete_left;

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
fn delete_left_fail_1() {
    let mut limbs: Vec<u32> = Vec::new();
    limbs_delete_left(&mut limbs, 1);
}

#[test]
#[should_panic]
fn delete_left_fail_2() {
    let mut limbs: Vec<u32> = vec![1, 2, 3];
    limbs_delete_left(&mut limbs, 4);
}
