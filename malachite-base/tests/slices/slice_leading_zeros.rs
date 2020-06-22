use malachite_base::slices::slice_leading_zeros;

#[test]
fn test_slice_leading_zeros() {
    let test = |xs: &[u32], out| {
        assert_eq!(slice_leading_zeros(xs), out);
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
