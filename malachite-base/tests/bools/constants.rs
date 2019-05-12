use malachite_base::comparison::{Max, Min};

#[test]
fn test_min() {
    assert_eq!(bool::MIN, false);
}

#[test]
fn test_max() {
    assert_eq!(bool::MAX, true);
}
