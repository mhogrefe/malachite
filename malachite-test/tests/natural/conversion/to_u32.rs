use common::LARGE_LIMIT;
use malachite_native::natural as native;
use malachite_gmp::natural as gmp;
use malachite_test::common::{gmp_to_native, native_to_rugint};
use rugint;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::naturals::{exhaustive_naturals, random_naturals};
use std::str::FromStr;

#[test]
fn test_to_u32() {
    let test = |n, out| {
        assert_eq!(native::Natural::from_str(n).unwrap().to_u32(), out);
        assert_eq!(gmp::Natural::from_str(n).unwrap().to_u32(), out);
        assert_eq!(rugint::Integer::from_str(n).unwrap().to_u32(), out);
    };
    test("0", Some(0));
    test("123", Some(123));
    test("1000000000000", None);
    test("4294967295", Some(4294967295));
    test("4294967296", None);
}

#[test]
fn test_to_u32_wrapping() {
    let test = |n, out| {
        assert_eq!(native::Natural::from_str(n).unwrap().to_u32_wrapping(), out);
        assert_eq!(gmp::Natural::from_str(n).unwrap().to_u32_wrapping(), out);
        assert_eq!(rugint::Integer::from_str(n).unwrap().to_u32_wrapping(), out);
    };
    test("0", 0);
    test("123", 123);
    test("1000000000000", 3567587328);
    test("4294967296", 0);
    test("4294967297", 1);
}

#[test]
fn to_u32_properties() {
    // x.to_u32() is equivalent for malachite-gmp, malachite-native, and rugint.
    // if x < 2^32, from(x.to_u32().unwrap()) == x
    // if x < 2^32, x.to_u32() == Some(x.to_u32_wrapping())
    // if x >= 2^32, x.to_u32().is_none()
    let one_natural = |gmp_x: gmp::Natural| {
        let x = gmp_to_native(&gmp_x);
        let native_u32 = x.to_u32();
        assert_eq!(gmp_x.to_u32(), native_u32);
        assert_eq!(native_to_rugint(&x).to_u32(), native_u32);
        if x.significant_bits() <= 32 {
            assert_eq!(native::Natural::from(native_u32.unwrap()), x);
            assert_eq!(native_u32, Some(x.to_u32_wrapping()));
        } else {
            assert!(native_u32.is_none());
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
fn to_u32_wrapping_properties() {
    // x.to_u32_wrapping() is equivalent for malachite-gmp, malachite-native, and rugint.
    // TODO relate with BitAnd
    let one_natural = |gmp_x: gmp::Natural| {
        let x = gmp_to_native(&gmp_x);
        let native_u32 = x.to_u32_wrapping();
        assert_eq!(gmp_x.to_u32_wrapping(), native_u32);
        assert_eq!(native_to_rugint(&x).to_u32_wrapping(), native_u32);
    };

    for n in exhaustive_naturals().take(LARGE_LIMIT) {
        one_natural(n);
    }

    for n in random_naturals(&EXAMPLE_SEED, 32).take(LARGE_LIMIT) {
        one_natural(n);
    }
}
