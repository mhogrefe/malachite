use malachite_base::iterators::comparison::{
    is_strictly_ascending, is_strictly_descending, is_weakly_descending,
};

fn is_strictly_descending_helper(xs: &[u8], result: bool) {
    assert_eq!(is_strictly_descending(xs.iter()), result);
    assert_eq!(
        is_strictly_ascending(xs.iter().rev().collect::<Vec<_>>().into_iter()),
        result
    );
    if result {
        assert!(is_weakly_descending(xs.iter()));
        if xs.len() > 1 {
            assert!(!is_strictly_ascending(xs.iter()));
        }
    }
}

#[test]
fn test_is_strictly_descending() {
    is_strictly_descending_helper(&[], true);
    is_strictly_descending_helper(&[5], true);
    is_strictly_descending_helper(&[6, 5], true);
    is_strictly_descending_helper(&[6, 6], false);
    is_strictly_descending_helper(&[6, 7], false);
    is_strictly_descending_helper(&[4, 3, 2, 1], true);
    is_strictly_descending_helper(&[4, 2, 2, 1], false);
    is_strictly_descending_helper(&[4, 2, 3, 1], false);
    is_strictly_descending_helper(&[3, 1, 4, 1, 5, 9], false);
}
