use malachite_base::iterators::first_and_last;

fn first_and_last_helper(xs: &[u8], result: Option<(u8, u8)>) {
    assert_eq!(first_and_last(&mut xs.iter().cloned()), result);
}

#[test]
fn test_first_and_last() {
    first_and_last_helper(&[1, 2, 10, 11, 12, 7, 8, 16, 5], Some((1, 5)));
    first_and_last_helper(&[5], Some((5, 5)));
    first_and_last_helper(&[], None);
}
