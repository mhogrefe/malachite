use malachite_base::iterators::nonzero_values;

#[test]
pub fn test_nonzero_values() {
    let test = |xs: &[u32], out: &[u32]| {
        assert_eq!(nonzero_values(xs.iter().cloned()).collect::<Vec<_>>(), out);
    };
    test(&[], &[]);
    test(&[1, 2, 3], &[1, 2, 3]);
    test(&[1, 0, 3], &[1, 3]);
    test(&[1, 2, 0, 0, 0], &[1, 2]);
    test(&[0, 0, 0], &[]);
}
