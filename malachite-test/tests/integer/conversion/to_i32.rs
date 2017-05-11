use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use rugint;
use std::str::FromStr;

#[test]
fn test_to_i32() {
    let test = |n, out| {
        assert_eq!(native::Integer::from_str(n).unwrap().to_i32(), out);
        assert_eq!(gmp::Integer::from_str(n).unwrap().to_i32(), out);
        assert_eq!(rugint::Integer::from_str(n).unwrap().to_i32(), out);
    };
    test("0", Some(0));
    test("123", Some(123));
    test("-123", Some(-123));
    test("1000000000000", None);
    test("-1000000000000", None);
    test("2147483647", Some(2147483647));
    test("2147483648", None);
    test("-2147483648", Some(-2147483648));
    test("-2147483649", None);
}

#[test]
fn test_to_i32_wrapping() {
    let test = |n, out| {
        assert_eq!(native::Integer::from_str(n).unwrap().to_i32_wrapping(), out);
        assert_eq!(gmp::Integer::from_str(n).unwrap().to_i32_wrapping(), out);
        assert_eq!(rugint::Integer::from_str(n).unwrap().to_i32_wrapping(), out);
    };
    test("0", 0);
    test("123", 123);
    test("-123", -123);
    test("1000000000000", -727379968);
    test("-1000000000000", 727379968);
    test("2147483647", 2147483647);
    test("2147483648", -2147483648);
    test("-2147483648", -2147483648);
    test("-2147483649", 2147483647);
}
