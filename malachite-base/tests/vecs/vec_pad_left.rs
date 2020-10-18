use malachite_base::vecs::vec_pad_left;

#[test]
fn test_vec_pad_left() {
    let test = |xs: &[u32], pad_size: usize, pad_value: u32, out: &[u32]| {
        let mut mut_xs = xs.to_vec();
        vec_pad_left(&mut mut_xs, pad_size, pad_value);
        assert_eq!(mut_xs, out);
    };
    test(&[], 3, 6, &[6, 6, 6]);
    test(&[1, 2, 3], 0, 10, &[1, 2, 3]);
    test(&[1, 2, 3], 5, 10, &[10, 10, 10, 10, 10, 1, 2, 3]);
}
