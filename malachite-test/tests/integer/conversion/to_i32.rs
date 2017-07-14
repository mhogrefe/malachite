use common::LARGE_LIMIT;
use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use malachite_test::common::{gmp_integer_to_native, native_integer_to_rugint};
use rugint;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
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

#[test]
fn to_i32_properties() {
    // x.to_i32() is equivalent for malachite-gmp, malachite-native, and rugint.
    // if -2^31 ≤ x < 2^31, from(x.to_i32().unwrap()) == x
    // if -2^31 ≤ x < 2^31, x.to_i32() == Some(x.to_i32_wrapping())
    // if x < -2^31 or x >= 2^31, x.to_i32().is_none()
    let one_integer = |gmp_x: gmp::Integer| {
        let x = gmp_integer_to_native(&gmp_x);
        let native_i32 = x.to_i32();
        assert_eq!(gmp_x.to_i32(), native_i32);
        assert_eq!(native_integer_to_rugint(&x).to_i32(), native_i32);
        if x >= i32::min_value() && x <= i32::max_value() {
            assert_eq!(native::Integer::from(native_i32.unwrap()), x);
            assert_eq!(native_i32, Some(x.to_i32_wrapping()));
        } else {
            assert!(native_i32.is_none());
        }
    };

    for n in exhaustive_integers().take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in random_integers(&EXAMPLE_SEED, 32).take(LARGE_LIMIT) {
        one_integer(n);
    }
}

#[test]
fn to_i32_wrapping_properties() {
    // x.to_i32_wrapping() is equivalent for malachite-gmp, malachite-native, and rugint.
    // (-x).to_i32_wrapping() = -(x.to_i32_wrapping())
    // TODO relate with BitAnd
    let one_integer = |gmp_x: gmp::Integer| {
        let x = gmp_integer_to_native(&gmp_x);
        let native_i32 = x.to_i32_wrapping();
        assert_eq!(gmp_x.to_i32_wrapping(), native_i32);
        assert_eq!(native_integer_to_rugint(&x).to_i32_wrapping(), native_i32);
        assert_eq!(-native_i32, (-&x).to_i32_wrapping());
    };

    for n in exhaustive_integers().take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in random_integers(&EXAMPLE_SEED, 32).take(LARGE_LIMIT) {
        one_integer(n);
    }
}
