use malachite_base::iterators::matching_intervals_in_iterator;

fn matching_intervals_in_iterator_helper<F: Fn(&u8) -> bool>(xs: &[u8], f: F, result: &[(u8, u8)]) {
    assert_eq!(
        matching_intervals_in_iterator(xs.iter().cloned(), f).as_slice(),
        result
    );
}

#[test]
fn test_matching_intervals_in_iterator() {
    let xs = &[1, 2, 10, 11, 12, 7, 8, 16, 5];
    matching_intervals_in_iterator_helper(xs, |&x| x >= 10, &[(10, 12), (16, 16)]);
    matching_intervals_in_iterator_helper(xs, |&x| x < 10, &[(1, 2), (7, 8), (5, 5)]);
    matching_intervals_in_iterator_helper(xs, |&x| x >= 100, &[]);
    matching_intervals_in_iterator_helper(xs, |&x| x < 100, &[(1, 5)]);
}
