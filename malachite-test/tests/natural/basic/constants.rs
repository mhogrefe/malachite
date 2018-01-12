use malachite_base::traits::{One, Two, Zero};
use malachite_nz::natural::Natural;

#[test]
fn test_zero() {
    let zero = Natural::ZERO;
    assert_eq!(zero.to_string(), "0");
    assert!(zero.is_valid());
}

#[test]
fn test_one() {
    let one = Natural::ONE;
    assert_eq!(one.to_string(), "1");
    assert!(one.is_valid());
}

#[test]
fn test_two() {
    let two = Natural::TWO;
    assert_eq!(two.to_string(), "2");
    assert!(two.is_valid());
}
