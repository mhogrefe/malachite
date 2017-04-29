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
