use common::test_properties;
use malachite_base::misc::{CheckedFrom, WrappingFrom};
use malachite_base::num::{One, PrimitiveInteger, SignificantBits};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_test::inputs::integer::integers;
use std::{i32, i64};
use std::str::FromStr;

#[test]
fn test_i64_checked_from_integer() {
    let test = |n, out| {
        assert_eq!(i64::checked_from(&Integer::from_str(n).unwrap()), out);
    };
    test("0", Some(0));
    test("123", Some(123));
    test("-123", Some(-123));
    test("1000000000000", Some(1_000_000_000_000));
    test("-1000000000000", Some(-1_000_000_000_000));
    test("1000000000000000000000", None);
    test("-1000000000000000000000", None);
    test("2147483647", Some(i32::MAX.into()));
    test("2147483648", Some(-i64::from(i32::MIN)));
    test("-2147483648", Some(i32::MIN.into()));
    test("-2147483649", Some(i64::from(i32::MIN) - 1));
    test("9223372036854775807", Some(i64::MAX));
    test("9223372036854775808", None);
    test("-9223372036854775808", Some(i64::MIN));
    test("-9223372036854775809", None);
}

#[test]
fn test_i64_wrapping_from_integer() {
    let test = |n, out| {
        assert_eq!(i64::wrapping_from(&Integer::from_str(n).unwrap()), out);
    };
    test("0", 0);
    test("123", 123);
    test("-123", -123);
    test("1000000000000", 1_000_000_000_000);
    test("-1000000000000", -1_000_000_000_000);
    test("1000000000000000000000", 3_875_820_019_684_212_736);
    test("-1000000000000000000000", -3_875_820_019_684_212_736);
    test("2147483647", i32::MAX.into());
    test("2147483648", -i64::from(i32::MIN));
    test("-2147483648", i32::MIN.into());
    test("-2147483649", i64::from(i32::MIN) - 1);
    test("9223372036854775807", i64::MAX);
    test("9223372036854775808", i64::MIN);
    test("-9223372036854775808", i64::MIN);
    test("-9223372036854775809", i64::MAX);
}

#[test]
fn i64_checked_from_integer_properties() {
    test_properties(integers, |x| {
        let result = i64::checked_from(x);
        if x.significant_bits() < u64::from(i64::WIDTH) || *x == -(Natural::ONE << (i64::WIDTH - 1))
        {
            assert_eq!(Integer::from(result.unwrap()), *x);
            assert_eq!(result, Some(i64::wrapping_from(x)));
        } else {
            assert!(result.is_none());
        }
    });
}

#[test]
fn i64_wrapping_from_integer_properties() {
    // TODO relate with BitAnd
    test_properties(integers, |x| {
        let result = i64::wrapping_from(x);
        assert_eq!(-result, i64::wrapping_from(&-x));
    });
}
