use malachite_base::iterators::is_unique;

fn is_unique_helper(xs: &[u8], result: bool) {
    assert_eq!(is_unique(xs.iter()), result);
    assert_eq!(is_unique(xs.iter().rev()), result);
}

#[test]
fn test_is_unique() {
    is_unique_helper(&[], true);
    is_unique_helper(&[5], true);
    is_unique_helper(&[5, 6], true);
    is_unique_helper(&[5, 5], false);
    is_unique_helper(&[5, 4], true);
    is_unique_helper(&[1, 2, 3, 4], true);
    is_unique_helper(&[1, 2, 2, 4], false);
    is_unique_helper(&[1, 2, 3, 1], false);
    is_unique_helper(&[4; 100], false);

    assert_eq!(is_unique([1, 2].iter().cycle()), false);
}
