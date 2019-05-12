use malachite_base::limbs::limbs_leading_zero_limbs;

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
