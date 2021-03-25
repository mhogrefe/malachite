use malachite_base_test_util::common::test_cmp_helper;
use malachite_nz::integer::Integer;
use num::BigInt;
use rug;

#[test]
fn test_ord() {
    let strings = &[
        "-1000000000001",
        "-1000000000000",
        "-999999999999",
        "-123",
        "-2",
        "-1",
        "0",
        "1",
        "2",
        "123",
        "999999999999",
        "1000000000000",
        "1000000000001",
    ];
    test_cmp_helper::<Integer>(strings);
    test_cmp_helper::<BigInt>(strings);
    test_cmp_helper::<rug::Integer>(strings);
}
