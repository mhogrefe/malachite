use malachite_nz::integer::Integer;

#[test]
fn test_default() {
    let default = Integer::default();
    assert!(default.is_valid());
    assert_eq!(default, 0);
    assert_eq!(default.to_string(), "0");
}
