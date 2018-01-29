use common::LARGE_LIMIT;
use malachite_base::num::SignificantBits;
use malachite_base::traits::One;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_test::common::GenerationMode;
use malachite_test::inputs::integer::integers;
use std::{i32, i64};
use std::str::FromStr;

#[test]
fn test_to_i64() {
    let test = |n, out| {
        assert_eq!(Integer::from_str(n).unwrap().to_i64(), out);
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
fn test_to_i64_wrapping() {
    let test = |n, out| {
        assert_eq!(Integer::from_str(n).unwrap().to_i64_wrapping(), out);
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
fn to_i64_properties() {
    // if -2^63 ≤ x < 2^63, from(x.to_i64().unwrap()) == x
    // if -2^63 ≤ x < 2^63, x.to_i64() == Some(x.to_i64_wrapping())
    // if x < -2^63 or x >= 2^63, x.to_i64().is_none()
    let one_integer = |x: Integer| {
        let result = x.to_i64();
        if x.significant_bits() < 64 || x == -((Natural::ONE << 63u32).into_integer()) {
            assert_eq!(Integer::from(result.unwrap()), x);
            assert_eq!(result, Some(x.to_i64_wrapping()));
        } else {
            assert!(result.is_none());
        }
    };

    for n in integers(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in integers(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_integer(n);
    }
}

#[test]
fn to_i32_wrapping_properties() {
    // (-x).to_i64_wrapping() = -(x.to_i64_wrapping())
    // TODO relate with BitAnd
    let one_integer = |x: Integer| {
        let result = x.to_i64_wrapping();
        assert_eq!(-result, (-&x).to_i64_wrapping());
    };

    for n in integers(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in integers(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_integer(n);
    }
}
