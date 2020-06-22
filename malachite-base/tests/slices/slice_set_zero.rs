use malachite_base::slices::slice_set_zero;

#[test]
fn test_slice_set_zero() {
    let test = |xs: &[u32], out: &[u32]| {
        let mut mut_xs = xs.to_vec();
        slice_set_zero(&mut mut_xs);
        assert_eq!(mut_xs, out);
    };
    test(&[], &[]);
    test(&[0], &[0]);
    test(&[0, 0, 0], &[0, 0, 0]);
    test(&[123], &[0]);
    test(&[123, 0], &[0, 0]);
    test(&[0, 123, 0, 0, 0], &[0, 0, 0, 0, 0]);
}
