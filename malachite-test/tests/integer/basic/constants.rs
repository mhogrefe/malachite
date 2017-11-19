use malachite_base::traits::{NegativeOne, One, Two, Zero};
use malachite_native::integer as native;
use malachite_gmp::integer as gmp;

#[test]
fn test_zero() {
    let zero = native::Integer::zero();
    assert_eq!(zero.to_string(), "0");
    assert!(zero.is_valid());

    let zero = gmp::Integer::zero();
    assert_eq!(zero.to_string(), "0");
    assert!(zero.is_valid());
}

#[test]
fn test_one() {
    let one = native::Integer::one();
    assert_eq!(one.to_string(), "1");
    assert!(one.is_valid());

    let one = gmp::Integer::one();
    assert_eq!(one.to_string(), "1");
    assert!(one.is_valid());
}

#[test]
fn test_two() {
    let two = native::Integer::two();
    assert_eq!(two.to_string(), "2");
    assert!(two.is_valid());

    let two = gmp::Integer::two();
    assert_eq!(two.to_string(), "2");
    assert!(two.is_valid());
}

#[test]
fn test_negative_one() {
    let negative_one = native::Integer::negative_one();
    assert_eq!(negative_one.to_string(), "-1");
    assert!(negative_one.is_valid());

    let negative_one = gmp::Integer::negative_one();
    assert_eq!(negative_one.to_string(), "-1");
    assert!(negative_one.is_valid());
}
