use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use num;

#[test]
fn test_default() {
    let default = native::Natural::default();
    assert_eq!(default.to_string(), "0");
    assert!(default.is_valid());

    let default = gmp::Natural::default();
    assert_eq!(default.to_string(), "0");
    assert!(default.is_valid());

    assert_eq!(num::BigUint::default().to_string(), "0");
}
