use malachite_nz::natural::Natural;

#[test]
fn test_default() {
    let default = Natural::default();
    assert!(default.is_valid());
    assert_eq!(default, 0);
    assert_eq!(default.to_string(), "0");
}
