use malachite_base::iterators::count_is_at_most;
use std::iter::repeat;

fn count_is_at_most_helper(xs: &[u8], n: usize, result: bool) {
    assert_eq!(count_is_at_most(xs.iter(), n), result);
    assert_eq!(count_is_at_most(xs.iter().rev(), n), result);
}

#[test]
fn test_count_is_at_most() {
    count_is_at_most_helper(&[], 0, true);
    count_is_at_most_helper(&[], 1, true);
    count_is_at_most_helper(&[5], 0, false);
    count_is_at_most_helper(&[5], 1, true);
    count_is_at_most_helper(&[5], 2, true);
    count_is_at_most_helper(&[1, 2, 3, 4], 3, false);
    count_is_at_most_helper(&[1, 2, 3, 4], 4, true);
    count_is_at_most_helper(&[1, 2, 3, 4], 5, true);
    count_is_at_most_helper(&[4; 100], 120, true);
    count_is_at_most_helper(&[4; 100], 90, false);

    assert_eq!(count_is_at_most(repeat(10), 20), false);
}
