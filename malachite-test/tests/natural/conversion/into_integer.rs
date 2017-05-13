use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use std::str::FromStr;

#[test]
fn test_into_integer() {
    let test = |n| {
        assert_eq!(native::Natural::from_str(n).unwrap().into_integer().to_string(),
                   n);
        assert_eq!(gmp::Natural::from_str(n).unwrap().into_integer().to_string(),
                   n);
    };
    test("0");
    test("123");
    test("1000000000000");
    test("4294967295");
    test("4294967296");
}
