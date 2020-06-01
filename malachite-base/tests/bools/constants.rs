use malachite_base::comparison::traits::{Max, Min};
use malachite_base::named::Named;

#[test]
fn test_min() {
    assert_eq!(bool::MIN, false);
}

#[test]
fn test_max() {
    assert_eq!(bool::MAX, true);
}

#[test]
pub fn test_named() {
    assert_eq!(bool::NAME, "bool");
}
