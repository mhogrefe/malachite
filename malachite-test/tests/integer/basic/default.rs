use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use num;
use rugint;

#[test]
fn test_default() {
    let default = native::Integer::default();
    assert_eq!(default.to_string(), "0");
    assert!(default.is_valid());

    let default = gmp::Integer::new();
    assert_eq!(default.to_string(), "0");
    assert!(default.is_valid());

    assert_eq!(num::BigInt::default().to_string(), "0");

    assert_eq!(rugint::Integer::default().to_string(), "0");
}
