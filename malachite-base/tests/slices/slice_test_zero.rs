use malachite_base::slices::slice_test_zero::slice_test_zero;

#[test]
fn test_slice_test_zero() {
    let test = |xs: &[u32], out| {
        assert_eq!(slice_test_zero(xs), out);
    };
    test(&[], true);
    test(&[0], true);
    test(&[0, 0, 0], true);
    test(&[123], false);
    test(&[123, 0], false);
    test(&[0, 123, 0, 0, 0], false);
}
