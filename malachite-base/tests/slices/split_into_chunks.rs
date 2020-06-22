use malachite_base::slices::slice_set_zero;

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

    split_into_chunks!(xs, 1, [xs_1, xs_2], xs_3);
    assert_eq!(xs_1, &[0]);
    assert_eq!(xs_2, &[1]);
    assert_eq!(xs_3, &[2, 3, 4, 5, 6, 7]);

    split_into_chunks!(xs, 0, [xs_1, xs_2], xs_3);
    assert_eq!(xs_1, &[]);
    assert_eq!(xs_2, &[]);
    assert_eq!(xs_3, &[0, 1, 2, 3, 4, 5, 6, 7]);

    split_into_chunks!(xs, 5, [], xs_1);
    assert_eq!(xs_1, &[0, 1, 2, 3, 4, 5, 6, 7]);
}

#[test]
#[should_panic]
fn split_into_chunks_fail() {
    let xs = &[0, 1, 2, 3, 4, 5, 6, 7];
    split_into_chunks!(xs, 5, [_xs_1, _xs_2], _xs_3);
}

#[test]
fn test_split_into_chunks_mut() {
    let xs = &mut [0, 1, 2, 3, 4, 5, 6, 7];
    split_into_chunks_mut!(xs, 3, [xs_1, xs_2], xs_3);
    assert_eq!(xs_1, &[0, 1, 2]);
    assert_eq!(xs_2, &[3, 4, 5]);
    assert_eq!(xs_3, &[6, 7]);

    split_into_chunks_mut!(xs, 3, [xs_1], xs_2);
    assert_eq!(xs_1, &[0, 1, 2]);
    assert_eq!(xs_2, &[3, 4, 5, 6, 7]);

    split_into_chunks_mut!(xs, 1, [xs_1, xs_2], xs_3);
    assert_eq!(xs_1, &[0]);
    assert_eq!(xs_2, &[1]);
    assert_eq!(xs_3, &[2, 3, 4, 5, 6, 7]);

    split_into_chunks_mut!(xs, 0, [xs_1, xs_2], xs_3);
    assert_eq!(xs_1, &[]);
    assert_eq!(xs_2, &[]);
    assert_eq!(xs_3, &[0, 1, 2, 3, 4, 5, 6, 7]);

    split_into_chunks_mut!(xs, 5, [], xs_1);
    assert_eq!(xs_1, &[0, 1, 2, 3, 4, 5, 6, 7]);

    split_into_chunks_mut!(xs, 3, [_xs_1, xs_2], _xs_3);
    slice_set_zero(xs_2);
    assert_eq!(xs, &[0, 1, 2, 0, 0, 0, 6, 7]);
}

#[test]
#[should_panic]
fn split_into_chunks_mut_fail() {
    let xs = &mut [0, 1, 2, 3, 4, 5, 6, 7];
    split_into_chunks_mut!(xs, 5, [_xs_1, _xs_2], _xs_3);
}
