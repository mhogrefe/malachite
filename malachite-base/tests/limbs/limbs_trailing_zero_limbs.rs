use malachite_base::limbs::limbs_trailing_zero_limbs;

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
