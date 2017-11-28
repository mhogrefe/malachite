use malachite_base::traits::{One, Two, Zero};
use malachite_native::natural as native;
use malachite_gmp::natural as gmp;

#[test]
fn test_zero() {
    let zero = native::Natural::ZERO;
    assert_eq!(zero.to_string(), "0");
    assert!(zero.is_valid());

    let zero = gmp::Natural::ZERO;
    assert_eq!(zero.to_string(), "0");
    assert!(zero.is_valid());
}

#[test]
fn test_one() {
    let one = native::Natural::ONE;
    assert_eq!(one.to_string(), "1");
    assert!(one.is_valid());

    let one = gmp::Natural::ONE;
    assert_eq!(one.to_string(), "1");
    assert!(one.is_valid());
}

#[test]
fn test_two() {
    let two = native::Natural::TWO;
    assert_eq!(two.to_string(), "2");
    assert!(two.is_valid());

    let two = gmp::Natural::TWO;
    assert_eq!(two.to_string(), "2");
    assert!(two.is_valid());
}
