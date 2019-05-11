use malachite_base::limbs::limbs_set_zero;

#[test]
fn test_limbs_test_zero() {
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
