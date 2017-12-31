use common::LARGE_LIMIT;
use malachite_base::traits::One;
use malachite_gmp::integer as gmp;
use malachite_native::integer as native;
use malachite_native::natural as native_natural;
use malachite_test::common::{gmp_integer_to_native, GenerationMode};
use malachite_test::integer::conversion::to_i64::select_inputs;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use std::str::FromStr;

#[test]
fn test_to_i64() {
    let test = |n, out| {
        assert_eq!(native::Integer::from_str(n).unwrap().to_i64(), out);
        assert_eq!(gmp::Integer::from_str(n).unwrap().to_i64(), out);
    };
    test("0", Some(0));
    test("123", Some(123));
    test("-123", Some(-123));
    test("1000000000000", Some(1000000000000));
    test("-1000000000000", Some(-1000000000000));
    test("1000000000000000000000", None);
    test("-1000000000000000000000", None);
    test("2147483647", Some(2147483647));
    test("2147483648", Some(2147483648));
    test("-2147483648", Some(-2147483648));
    test("-2147483649", Some(-2147483649));
    test("9223372036854775807", Some(9223372036854775807));
    test("9223372036854775808", None);
    test("-9223372036854775808", Some(-9223372036854775808));
    test("-9223372036854775809", None);
}

#[test]
fn test_to_i64_wrapping() {
    let test = |n, out| {
        assert_eq!(native::Integer::from_str(n).unwrap().to_i64_wrapping(), out);
        assert_eq!(gmp::Integer::from_str(n).unwrap().to_i64_wrapping(), out);
    };
    test("0", 0);
    test("123", 123);
    test("-123", -123);
    test("1000000000000", 1000000000000);
    test("-1000000000000", -1000000000000);
    test("1000000000000000000000", 3875820019684212736);
    test("-1000000000000000000000", -3875820019684212736);
    test("2147483647", 2147483647);
    test("2147483648", 2147483648);
    test("-2147483648", -2147483648);
    test("-2147483649", -2147483649);
    test("9223372036854775807", 9223372036854775807);
    test("9223372036854775808", -9223372036854775808);
    test("-9223372036854775808", -9223372036854775808);
    test("-9223372036854775809", 9223372036854775807);
}

#[test]
fn to_i64_properties() {
    // x.to_i64() is equivalent for malachite-gmp and malachite-native.
    // if -2^63 ≤ x < 2^63, from(x.to_i64().unwrap()) == x
    // if -2^63 ≤ x < 2^63, x.to_i64() == Some(x.to_i64_wrapping())
    // if x < -2^63 or x >= 2^63, x.to_i64().is_none()
    let one_integer = |gmp_x: gmp::Integer| {
        let x = gmp_integer_to_native(&gmp_x);
        let native_i64 = x.to_i64();
        assert_eq!(gmp_x.to_i64(), native_i64);
        if x.significant_bits() < 64
            || x == -((native_natural::Natural::ONE << 63u32).into_integer())
        {
            assert_eq!(native::Integer::from(native_i64.unwrap()), x);
            assert_eq!(native_i64, Some(x.to_i64_wrapping()));
        } else {
            assert!(native_i64.is_none());
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
    // x.to_i64_wrapping() is equivalent for malachite-gmp and malachite-native.
    // (-x).to_i64_wrapping() = -(x.to_i64_wrapping())
    // TODO relate with BitAnd
    let one_integer = |gmp_x: gmp::Integer| {
        let x = gmp_integer_to_native(&gmp_x);
        let native_i64 = x.to_i64_wrapping();
        assert_eq!(gmp_x.to_i64_wrapping(), native_i64);
        assert_eq!(-native_i64, (-&x).to_i64_wrapping());
    };

    for n in select_inputs(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in select_inputs(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_integer(n);
    }
}
