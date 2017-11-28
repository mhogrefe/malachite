use malachite_base::traits::{NegativeOne, One, Two, Zero};
use malachite_native::integer as native;
use malachite_gmp::integer as gmp;

#[test]
fn test_zero() {
    assert_eq!(native::Integer::ZERO.to_string(), "0");
    assert!(native::Integer::ZERO.is_valid());

    assert_eq!(gmp::Integer::ZERO.to_string(), "0");
    assert!(gmp::Integer::ZERO.is_valid());
}

#[test]
fn test_one() {
    assert_eq!(native::Integer::ONE.to_string(), "1");
    assert!(native::Integer::ONE.is_valid());

    assert_eq!(gmp::Integer::ONE.to_string(), "1");
    assert!(gmp::Integer::ONE.is_valid());
}

#[test]
fn test_two() {
    assert_eq!(native::Integer::TWO.to_string(), "2");
    assert!(native::Integer::TWO.is_valid());

    assert_eq!(gmp::Integer::TWO.to_string(), "2");
    assert!(gmp::Integer::TWO.is_valid());
}

#[test]
fn test_negative_one() {
    assert_eq!(native::Integer::NEGATIVE_ONE.to_string(), "-1");
    assert!(native::Integer::NEGATIVE_ONE.is_valid());

    assert_eq!(gmp::Integer::NEGATIVE_ONE.to_string(), "-1");
    assert!(gmp::Integer::NEGATIVE_ONE.is_valid());
}
