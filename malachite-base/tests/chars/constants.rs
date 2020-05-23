use malachite_base::comparison::traits::{Max, Min};

#[test]
fn test_min() {
    assert_eq!(char::MIN, '\u{0}');
}

#[test]
fn test_max() {
    assert_eq!(char::MAX, '\u{10ffff}');
}
