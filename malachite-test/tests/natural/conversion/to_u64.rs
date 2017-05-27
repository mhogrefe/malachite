use common::LARGE_LIMIT;
use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use malachite_test::common::gmp_to_native;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use std::str::FromStr;

#[test]
fn test_to_u64() {
    let test = |n, out| {
        assert_eq!(native::Natural::from_str(n).unwrap().to_u64(), out);
        assert_eq!(gmp::Natural::from_str(n).unwrap().to_u64(), out);
    };
    test("0", Some(0));
    test("123", Some(123));
    test("1000000000000000000000", None);
    test("18446744073709551615", Some(18446744073709551615));
    test("18446744073709551616", None);
}

#[test]
fn test_to_u64_wrapping() {
    let test = |n, out| {
        assert_eq!(native::Natural::from_str(n).unwrap().to_u64_wrapping(), out);
        assert_eq!(gmp::Natural::from_str(n).unwrap().to_u64_wrapping(), out);
    };
    test("0", 0);
    test("123", 123);
    test("1000000000000000000000", 3875820019684212736u64);
    test("18446744073709551616", 0);
    test("18446744073709551617", 1);
}

#[test]
fn to_u64_properties() {
    // x.to_u64() is equivalent for malachite-gmp and malachite-native.
    // if x < 2^64, from(x.to_u64().unwrap()) == x
    // if x < 2^64, x.to_u64() == Some(x.to_u64_wrapping())
    // if x >= 2^64, x.to_u64().is_none()
    let one_natural = |gmp_x: gmp::Natural| {
        let x = gmp_to_native(&gmp_x);
        let native_u64 = x.to_u64();
        assert_eq!(gmp_x.to_u64(), native_u64);
        if x.significant_bits() <= 64 {
            assert_eq!(native::Natural::from(native_u64.unwrap()), x);
            assert_eq!(native_u64, Some(x.to_u64_wrapping()));
        } else {
            assert!(native_u64.is_none());
        }
    };

    for n in exhaustive_naturals().take(LARGE_LIMIT) {
        one_natural(n);
    }

    for n in random_naturals(&EXAMPLE_SEED, 32).take(LARGE_LIMIT) {
        one_natural(n);
    }
}

#[test]
fn to_u64_wrapping_properties() {
    // x.to_u64_wrapping() is equivalent for malachite-gmp and malachite-native.
    // TODO relate with BitAnd
    let one_natural = |gmp_x: gmp::Natural| {
        let x = gmp_to_native(&gmp_x);
        let native_u64 = x.to_u64_wrapping();
        assert_eq!(gmp_x.to_u64_wrapping(), native_u64);
    };

    for n in exhaustive_naturals().take(LARGE_LIMIT) {
        one_natural(n);
    }

    for n in random_naturals(&EXAMPLE_SEED, 32).take(LARGE_LIMIT) {
        one_natural(n);
    }
}
