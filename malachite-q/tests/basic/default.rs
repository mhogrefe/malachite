use malachite_q::Rational;

#[test]
fn test_default() {
    let default = Rational::default();
    assert!(default.is_valid());
    assert_eq!(default, 0);
    assert_eq!(default.to_string(), "0");
}
