use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use malachite_test::integer::arithmetic::sub_i32::num_sub_i32;
use num;
use rugint;
use std::str::FromStr;

#[test]
fn test_sub_assign_i32() {
    let test = |u, v: i32, out| {
        let mut n = native::Integer::from_str(u).unwrap();
        n -= v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = gmp::Integer::from_str(u).unwrap();
        n -= v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = rugint::Integer::from_str(u).unwrap();
        n -= v;
        assert_eq!(n.to_string(), out);
    };
    test("0", -123, "123");
    test("123", 0, "123");
    test("123", -456, "579");
    test("-123", -456, "333");
    test("-500", -456, "-44");
    test("123", 123, "0");
    test("456", 123, "333");
    test("123", 456, "-333");
    test("-456", 123, "-579");
    test("1000000000000", -123, "1000000000123");
    test("-1000000000000", -123, "-999999999877");
    test("1000000000000", 123, "999999999877");
    test("-1000000000000", 123, "-1000000000123");
    test("4294967295", -1, "4294967296");
    test("-4294967296", -1, "-4294967295");
    test("2147483647", -1, "2147483648");
    test("-2147483648", -1, "-2147483647");
    test("18446744073709551615", -1, "18446744073709551616");
    test("-18446744073709551616", -1, "-18446744073709551615");
    test("4294967296", 1, "4294967295");
    test("-4294967295", 1, "-4294967296");
    test("2147483648", 1, "2147483647");
    test("-2147483647", 1, "-2147483648");
    test("18446744073709551616", 1, "18446744073709551615");
    test("-18446744073709551615", 1, "-18446744073709551616");
}

#[test]
fn test_sub_i32() {
    let test = |u, v: i32, out| {
        let n = native::Integer::from_str(u).unwrap() - v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = gmp::Integer::from_str(u).unwrap() - v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = num_sub_i32(num::BigInt::from_str(u).unwrap(), v);
        assert_eq!(n.to_string(), out);

        let n = rugint::Integer::from_str(u).unwrap() - v;
        assert_eq!(n.to_string(), out);
    };
    test("0", -123, "123");
    test("123", 0, "123");
    test("123", -456, "579");
    test("-123", -456, "333");
    test("-500", -456, "-44");
    test("123", 123, "0");
    test("456", 123, "333");
    test("123", 456, "-333");
    test("-456", 123, "-579");
    test("1000000000000", -123, "1000000000123");
    test("-1000000000000", -123, "-999999999877");
    test("1000000000000", 123, "999999999877");
    test("-1000000000000", 123, "-1000000000123");
    test("4294967295", -1, "4294967296");
    test("-4294967296", -1, "-4294967295");
    test("2147483647", -1, "2147483648");
    test("-2147483648", -1, "-2147483647");
    test("18446744073709551615", -1, "18446744073709551616");
    test("-18446744073709551616", -1, "-18446744073709551615");
    test("4294967296", 1, "4294967295");
    test("-4294967295", 1, "-4294967296");
    test("2147483648", 1, "2147483647");
    test("-2147483647", 1, "-2147483648");
    test("18446744073709551616", 1, "18446744073709551615");
    test("-18446744073709551615", 1, "-18446744073709551616");
}

#[test]
fn test_i32_sub_integer() {
    let test = |u: i32, v, out| {
        let n = u - native::Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u - gmp::Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u - rugint::Integer::from_str(v).unwrap();
        assert_eq!(n.to_string(), out);
    };
    test(0, "-123", "123");
    test(123, "0", "123");
    test(123, "-456", "579");
    test(-123, "-456", "333");
    test(-500, "-456", "-44");
    test(123, "123", "0");
    test(456, "123", "333");
    test(123, "456", "-333");
    test(-456, "123", "-579");
    test(2147483647, "-1", "2147483648");
    test(-2147483648, "-1", "-2147483647");
    test(-2147483647, "1", "-2147483648");
}
