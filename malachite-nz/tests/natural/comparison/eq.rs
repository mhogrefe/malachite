use malachite_base_test_util::common::test_eq_helper;
use malachite_nz::natural::Natural;
use num::BigUint;
use rug;

#[test]
fn test_eq() {
    let strings = vec!["0", "1", "2", "123", "1000000000000"];
    test_eq_helper::<Natural>(&strings);
    test_eq_helper::<BigUint>(&strings);
    test_eq_helper::<rug::Integer>(&strings);
}
