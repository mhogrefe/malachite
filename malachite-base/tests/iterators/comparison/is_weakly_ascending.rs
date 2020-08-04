use malachite_base::iterators::comparison::{
    is_strictly_ascending, is_weakly_ascending, is_weakly_descending,
};

fn is_weakly_ascending_helper(xs: &[u8], result: bool) {
    assert_eq!(is_weakly_ascending(xs.iter()), result);
    assert_eq!(
        is_weakly_descending(xs.iter().rev().collect::<Vec<_>>().into_iter()),
        result
    );
    if !result {
        assert!(!is_strictly_ascending(xs.iter()));
    }
}

#[test]
fn test_is_weakly_ascending() {
    is_weakly_ascending_helper(&[], true);
    is_weakly_ascending_helper(&[5], true);
    is_weakly_ascending_helper(&[5, 6], true);
    is_weakly_ascending_helper(&[5, 5], true);
    is_weakly_ascending_helper(&[5, 4], false);
    is_weakly_ascending_helper(&[1, 2, 3, 4], true);
    is_weakly_ascending_helper(&[1, 2, 2, 4], true);
    is_weakly_ascending_helper(&[1, 3, 2, 4], false);
    is_weakly_ascending_helper(&[3, 1, 4, 1, 5, 9], false);
}
