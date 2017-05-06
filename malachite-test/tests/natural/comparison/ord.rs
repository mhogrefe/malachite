use common::test_ord_helper;
use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use num;

#[test]
fn test_ord() {
    let strings = vec!["0", "1", "2", "123", "999999999999", "1000000000000", "1000000000001"];
    test_ord_helper::<native::Natural>(&strings);
    test_ord_helper::<gmp::Natural>(&strings);
    test_ord_helper::<num::BigUint>(&strings);
}
