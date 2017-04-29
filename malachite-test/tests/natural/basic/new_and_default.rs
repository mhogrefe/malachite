use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use num::{self, Zero};

#[test]
fn test_new() {
    let new = native::Natural::new();
    assert_eq!(new.to_string(), "0");
    assert!(new.is_valid());

    let new = gmp::Natural::new();
    assert_eq!(new.to_string(), "0");
    assert!(new.is_valid());

    assert_eq!(num::BigUint::zero().to_string(), "0");
}

#[test]
fn test_default() {
    let default = native::Natural::default();
    assert_eq!(default.to_string(), "0");
    assert!(default.is_valid());

    let default = gmp::Natural::new();
    assert_eq!(default.to_string(), "0");
    assert!(default.is_valid());

    assert_eq!(num::BigUint::default().to_string(), "0");
}
