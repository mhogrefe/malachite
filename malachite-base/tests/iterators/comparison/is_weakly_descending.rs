use malachite_base::iterators::comparison::{
    is_strictly_descending, is_weakly_ascending, is_weakly_descending,
};

fn is_weakly_descending_helper(xs: &[u8], result: bool) {
    assert_eq!(is_weakly_descending(xs.iter()), result);
    assert_eq!(is_weakly_ascending(xs.iter().rev()), result);
    if xs.len() < 2 {
        assert!(result);
    }
    if !result {
        assert!(!is_strictly_descending(xs.iter()));
    }
}

#[test]
fn test_is_weakly_descending() {
    is_weakly_descending_helper(&[], true);
    is_weakly_descending_helper(&[5], true);
    is_weakly_descending_helper(&[6, 5], true);
    is_weakly_descending_helper(&[6, 6], true);
    is_weakly_descending_helper(&[6, 7], false);
    is_weakly_descending_helper(&[4, 3, 2, 1], true);
    is_weakly_descending_helper(&[4, 2, 2, 1], true);
    is_weakly_descending_helper(&[4, 2, 3, 1], false);
    is_weakly_descending_helper(&[3, 1, 4, 1, 5, 9], false);
}
