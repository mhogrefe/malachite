use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use std::str::FromStr;

#[test]
fn test_is_power_of_two() {
    let test = |n, out| {
        assert_eq!(native::Natural::from_str(n).unwrap().is_power_of_two(), out);
        assert_eq!(gmp::Natural::from_str(n).unwrap().is_power_of_two(), out);
    };
    test("0", false);
    test("1", true);
    test("2", true);
    test("3", false);
    test("4", true);
    test("5", false);
    test("6", false);
    test("7", false);
    test("8", true);
    test("1024", true);
    test("1025", false);
    test("1000000000000", false);
    test("1099511627776", true);
}
