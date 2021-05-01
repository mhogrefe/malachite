extern crate serde;
extern crate serde_json;
use malachite_nz::natural::Natural;
use std::str::FromStr;

#[test]
fn test_serde() {
    let test = |n, out| {
        assert_eq!(
            serde_json::to_string(&Natural::from_str(n).unwrap()).unwrap(),
            out
        );
        assert_eq!(serde_json::from_str::<Natural>(out).unwrap().to_string(), n);
    };
    test("0", "\"0x0\"");
    test("100", "\"0x64\"");
    test("1000000000000", "\"0xe8d4a51000\"");
    test("4294967295", "\"0xffffffff\"");
    test("4294967296", "\"0x100000000\"");
    test("18446744073709551615", "\"0xffffffffffffffff\"");
    test("18446744073709551616", "\"0x10000000000000000\"");
    test("1000000000000000000000000", "\"0xd3c21bcecceda1000000\"");
    test(
        "340282366920938463463374607431768211455",
        "\"0xffffffffffffffffffffffffffffffff\"",
    );
    test(
        "340282366920938463463374607431768211456",
        "\"0x100000000000000000000000000000000\"",
    );
}
