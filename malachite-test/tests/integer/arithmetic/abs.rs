use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use num::{self, Signed};
use rugint;
use std::str::FromStr;

#[test]
fn test_abs() {
    let test = |s, out| {
        assert_eq!(native::Integer::from_str(s).unwrap().abs().to_string(), out);
        assert_eq!(gmp::Integer::from_str(s).unwrap().abs().to_string(), out);
        assert_eq!(num::BigInt::from_str(s).unwrap().abs().to_string(), out);
        assert_eq!(rugint::Integer::from_str(s).unwrap().abs().to_string(), out);
    };
    test("0", "0");
    test("123", "123");
    test("-123", "123");
    test("1000000000000", "1000000000000");
    test("-1000000000000", "1000000000000");
    test("-2147483648", "2147483648");
}

#[test]
fn test_unsigned_abs() {
    let test = |s, out| {
        assert_eq!(native::Integer::from_str(s).unwrap().unsigned_abs().to_string(),
                   out);
        assert_eq!(gmp::Integer::from_str(s).unwrap().unsigned_abs().to_string(),
                   out);
    };
    test("0", "0");
    test("123", "123");
    test("-123", "123");
    test("1000000000000", "1000000000000");
    test("-1000000000000", "1000000000000");
    test("-2147483648", "2147483648");
    test("3000000000", "3000000000");
    test("-3000000000", "3000000000");
}
