use malachite_base::comparison::traits::Min;
use malachite_base::named::Named;

#[test]
fn test_min() {
    assert_eq!(char::MIN, '\u{0}');
}

#[test]
fn test_max() {
    assert_eq!(char::MAX, '\u{10ffff}');
}

#[test]
pub fn test_named() {
    assert_eq!(bool::NAME, "bool");
}
