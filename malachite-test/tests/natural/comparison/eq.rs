use common::test_eq_helper;
use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use num;

#[test]
fn test_eq() {
    let strings = vec!["0", "1", "2", "123", "1000000000000"];
    test_eq_helper::<native::Natural>(&strings);
    test_eq_helper::<gmp::Natural>(&strings);
    test_eq_helper::<num::BigUint>(&strings);
}
