use malachite_base_test_util::common::test_eq_helper;
use malachite_nz::integer::Integer;
use num::BigInt;
use rug;

#[test]
fn test_eq() {
    let strings = &[
        "0",
        "1",
        "-1",
        "2",
        "-2",
        "123",
        "-123",
        "1000000000000",
        "-1000000000000",
    ];
    test_eq_helper::<Integer>(strings);
    test_eq_helper::<BigInt>(strings);
    test_eq_helper::<rug::Integer>(strings);
}
