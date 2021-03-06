use malachite_base::comparison::traits::Min;
use malachite_base::num::basic::traits::{One, Two, Zero};
use malachite_nz::natural::Natural;

#[test]
fn test_min() {
    let min = Natural::MIN;
    assert!(min.is_valid());
    assert_eq!(min, 0);
    assert_eq!(min.to_string(), "0");
}

#[test]
fn test_zero() {
    let zero = Natural::ZERO;
    assert!(zero.is_valid());
    assert_eq!(zero, 0);
    assert_eq!(zero.to_string(), "0");
}

#[test]
fn test_one() {
    let one = Natural::ONE;
    assert!(one.is_valid());
    assert_eq!(one, 1);
    assert_eq!(one.to_string(), "1");
}

#[test]
fn test_two() {
    let two = Natural::TWO;
    assert!(two.is_valid());
    assert_eq!(two, 2);
    assert_eq!(two.to_string(), "2");
}
