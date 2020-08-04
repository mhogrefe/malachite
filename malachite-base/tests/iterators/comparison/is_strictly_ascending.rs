use malachite_base::iterators::comparison::{
    is_strictly_ascending, is_strictly_descending, is_weakly_ascending,
};

fn is_strictly_ascending_helper(xs: &[u8], result: bool) {
    assert_eq!(is_strictly_ascending(xs.iter()), result);
    assert_eq!(
        is_strictly_descending(xs.iter().rev().collect::<Vec<_>>().into_iter()),
        result
    );
    if result {
        assert!(is_weakly_ascending(xs.iter()));
        if xs.len() > 1 {
            assert!(!is_strictly_descending(xs.iter()));
        }
    }
}

#[test]
fn test_is_strictly_ascending() {
    is_strictly_ascending_helper(&[], true);
    is_strictly_ascending_helper(&[5], true);
    is_strictly_ascending_helper(&[5, 6], true);
    is_strictly_ascending_helper(&[5, 5], false);
    is_strictly_ascending_helper(&[5, 4], false);
    is_strictly_ascending_helper(&[1, 2, 3, 4], true);
    is_strictly_ascending_helper(&[1, 2, 2, 4], false);
    is_strictly_ascending_helper(&[1, 3, 2, 4], false);
    is_strictly_ascending_helper(&[3, 1, 4, 1, 5, 9], false);
}
