use malachite_base::strings::ToDebugString;

#[test]
pub fn test_to_debug_string() {
    let empty: &[Option<bool>] = &[];
    assert_eq!(empty.to_debug_string(), "[]");
    assert_eq!([1, 2, 3].to_debug_string(), "[1, 2, 3]");
    assert_eq!(
        [vec![2, 3], vec![], vec![4]].to_debug_string(),
        "[[2, 3], [], [4]]"
    );
    assert_eq!(Some(5).to_debug_string(), "Some(5)");
}
