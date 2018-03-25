use common::test_properties;
use malachite_nz::integer::Integer;
use malachite_base::misc::{CheckedFrom, WrappingFrom};
use malachite_test::common::integer_to_rug_integer;
use malachite_test::inputs::integer::integers;
use rug;
use std::i32;
use std::str::FromStr;

#[test]
fn test_i32_checked_from_integer() {
    let test = |n, out| {
        assert_eq!(i32::checked_from(&Integer::from_str(n).unwrap()), out);
        assert_eq!(rug::Integer::from_str(n).unwrap().to_i32(), out);
    };
    test("0", Some(0));
    test("123", Some(123));
    test("-123", Some(-123));
    test("1000000000000", None);
    test("-1000000000000", None);
    test("2147483647", Some(i32::MAX));
    test("2147483648", None);
    test("-2147483648", Some(i32::MIN));
    test("-2147483649", None);
}

#[test]
fn test_i32_wrapping_from_integer() {
    let test = |n, out| {
        assert_eq!(i32::wrapping_from(&Integer::from_str(n).unwrap()), out);
        assert_eq!(rug::Integer::from_str(n).unwrap().to_i32_wrapping(), out);
    };
    test("0", 0);
    test("123", 123);
    test("-123", -123);
    test("1000000000000", -727_379_968);
    test("-1000000000000", 727_379_968);
    test("2147483647", i32::MAX);
    test("2147483648", i32::MIN);
    test("-2147483648", i32::MIN);
    test("-2147483649", i32::MAX);
}

#[test]
fn i32_checked_from_integer_properties() {
    test_properties(integers, |x| {
        let result = i32::checked_from(x);
        assert_eq!(integer_to_rug_integer(x).to_i32(), result);
        if *x >= i32::MIN && *x <= i32::MAX {
            assert_eq!(Integer::from(result.unwrap()), *x);
            assert_eq!(result, Some(i32::wrapping_from(x)));
        } else {
            assert!(result.is_none());
        }
    });
}

#[test]
fn i32_wrapping_from_integer_properties() {
    // TODO relate with BitAnd
    test_properties(integers, |x| {
        let result = i32::wrapping_from(x);
        assert_eq!(integer_to_rug_integer(x).to_i32_wrapping(), result);
        assert_eq!(-result, i32::wrapping_from(-x));
    });
}
