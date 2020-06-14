use malachite_base::slices::slice_trailing_zeros::slice_trailing_zeros;

#[test]
fn test_slice_trailing_zeros() {
    let test = |xs: &[u32], out| {
        assert_eq!(slice_trailing_zeros(xs), out);
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
