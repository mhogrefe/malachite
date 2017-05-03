use common::test_eq_helper;
use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use num;
use rugint;

#[test]
fn test_eq() {
    let strings = vec!["0", "1", "-1", "2", "-2", "123", "-123", "1000000000000", "-1000000000000"];
    test_eq_helper::<native::Integer>(&strings);
    test_eq_helper::<gmp::Integer>(&strings);
    test_eq_helper::<num::BigInt>(&strings);
    test_eq_helper::<rugint::Integer>(&strings);
}
