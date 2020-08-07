use malachite_base::iterators::comparison::{is_strictly_zigzagging, is_weakly_zigzagging};

fn is_weakly_zigzagging_helper(xs: &[u8], result: bool) {
    assert_eq!(is_weakly_zigzagging(xs.iter()), result);
    assert_eq!(is_weakly_zigzagging(xs.iter().rev()), result);
    if xs.len() < 3 {
        assert!(result);
    }
    if !result {
        assert!(!is_strictly_zigzagging(xs.iter()));
    }
}

#[test]
fn test_is_weakly_zigzagging() {
    is_weakly_zigzagging_helper(&[], true);
    is_weakly_zigzagging_helper(&[5], true);
    is_weakly_zigzagging_helper(&[5, 6], true);
    is_weakly_zigzagging_helper(&[5, 5], true);
    is_weakly_zigzagging_helper(&[5, 4], true);
    is_weakly_zigzagging_helper(&[1, 2, 3, 4], false);
    is_weakly_zigzagging_helper(&[1, 2, 2, 4], true);
    is_weakly_zigzagging_helper(&[1, 3, 2, 4], true);
    is_weakly_zigzagging_helper(&[3, 1, 4, 1, 5, 9], false);
}
