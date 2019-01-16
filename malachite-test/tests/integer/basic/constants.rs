use malachite_base::num::{NegativeOne, One, Two, Zero};
use malachite_nz::integer::Integer;
use malachite_nz::platform::{Limb, SignedLimb};

#[test]
fn test_zero() {
    let zero = Integer::ZERO;
    assert!(zero.is_valid());
    assert_eq!(zero, 0 as Limb);
    assert_eq!(zero.to_string(), "0");
}

#[test]
fn test_one() {
    let one = Integer::ONE;
    assert!(one.is_valid());
    assert_eq!(one, 1 as Limb);
    assert_eq!(one.to_string(), "1");
}

#[test]
fn test_two() {
    let two = Integer::TWO;
    assert!(two.is_valid());
    assert_eq!(two, 2 as Limb);
    assert_eq!(two.to_string(), "2");
}

#[test]
fn test_negative_one() {
    let negative_one = Integer::NEGATIVE_ONE;
    assert!(negative_one.is_valid());
    assert_eq!(negative_one, -1 as SignedLimb);
    assert_eq!(negative_one.to_string(), "-1");
}
