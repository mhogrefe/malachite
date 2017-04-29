use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use num::{self, Zero};
use rugint;

#[test]
fn test_new() {
    let new = native::Integer::new();
    assert_eq!(new.to_string(), "0");
    assert!(new.is_valid());

    let new = gmp::Integer::new();
    assert_eq!(new.to_string(), "0");
    assert!(new.is_valid());

    assert_eq!(num::BigInt::zero().to_string(), "0");

    assert_eq!(rugint::Integer::new().to_string(), "0");
}
