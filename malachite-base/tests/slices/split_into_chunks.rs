//TODO

#[test]
fn test_split_into_chunks() {
    let xs = &[0, 1, 2, 3, 4, 5, 6, 7];
    split_into_chunks!(xs, 3, [xs_1, xs_2], xs_3);
    assert_eq!(xs_1, &[0, 1, 2]);
    assert_eq!(xs_2, &[3, 4, 5]);
    assert_eq!(xs_3, &[6, 7]);

    split_into_chunks!(xs, 3, [xs_1], xs_2);
    assert_eq!(xs_1, &[0, 1, 2]);
    assert_eq!(xs_2, &[3, 4, 5, 6, 7]);
}
