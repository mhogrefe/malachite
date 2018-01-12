use malachite_base::traits::{NegativeOne, One, Two, Zero};
use malachite_nz::integer::Integer;

#[test]
fn test_zero() {
    assert_eq!(Integer::ZERO.to_string(), "0");
    assert!(Integer::ZERO.is_valid());
}

#[test]
fn test_one() {
    assert_eq!(Integer::ONE.to_string(), "1");
    assert!(Integer::ONE.is_valid());
}

#[test]
fn test_two() {
    assert_eq!(Integer::TWO.to_string(), "2");
    assert!(Integer::TWO.is_valid());
}

#[test]
fn test_negative_one() {
    assert_eq!(Integer::NEGATIVE_ONE.to_string(), "-1");
    assert!(Integer::NEGATIVE_ONE.is_valid());
}
