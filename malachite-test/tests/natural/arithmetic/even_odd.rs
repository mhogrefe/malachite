use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use std::str::FromStr;

#[test]
fn test_is_even() {
    let test = |n, out| {
        assert_eq!(native::Natural::from_str(n).unwrap().is_even(), out);
        assert_eq!(gmp::Natural::from_str(n).unwrap().is_even(), out);
    };
    test("0", true);
    test("1", false);
    test("2", true);
    test("3", false);
    test("123", false);
    test("1000000000000", true);
    test("1000000000001", false);
}

#[test]
fn test_is_odd() {
    let test = |n, out| {
        assert_eq!(native::Natural::from_str(n).unwrap().is_odd(), out);
        assert_eq!(gmp::Natural::from_str(n).unwrap().is_odd(), out);
    };
    test("0", false);
    test("1", true);
    test("2", false);
    test("3", true);
    test("123", true);
    test("1000000000000", false);
    test("1000000000001", true);
}
