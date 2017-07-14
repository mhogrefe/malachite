use common::LARGE_LIMIT;
use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use malachite_test::common::{gmp_integer_to_native, native_integer_to_rugint};
use rugint;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use std::cmp::Ordering;
use std::str::FromStr;

#[test]
fn test_to_u32() {
    let test = |n, out| {
        assert_eq!(native::Integer::from_str(n).unwrap().to_u32(), out);
        assert_eq!(gmp::Integer::from_str(n).unwrap().to_u32(), out);
        assert_eq!(rugint::Integer::from_str(n).unwrap().to_u32(), out);
    };
    test("0", Some(0));
    test("123", Some(123));
    test("-123", None);
    test("1000000000000", None);
    test("-1000000000000", None);
    test("4294967295", Some(4294967295));
    test("4294967296", None);
}

#[test]
fn test_to_u32_wrapping() {
    let test = |n, out| {
        assert_eq!(native::Integer::from_str(n).unwrap().to_u32_wrapping(), out);
        assert_eq!(gmp::Integer::from_str(n).unwrap().to_u32_wrapping(), out);
        assert_eq!(rugint::Integer::from_str(n).unwrap().to_u32_wrapping(), out);
    };
    test("0", 0);
    test("123", 123);
    test("-123", 4294967173);
    test("1000000000000", 3567587328);
    test("-1000000000000", 727379968);
    test("4294967296", 0);
    test("4294967297", 1);
    test("-4294967296", 0);
    test("-4294967295", 1);
}

#[test]
fn to_u32_properties() {
    // x.to_u32() is equivalent for malachite-gmp, malachite-native, and rugint.
    // if 0 ≤ x < 2^32, from(x.to_u32().unwrap()) == x
    // if 0 ≤ x < 2^32, x.to_u32() == Some(x.to_u32_wrapping())
    // if x < 0 or x >= 2^32, x.to_u32().is_none()
    let one_integer = |gmp_x: gmp::Integer| {
        let x = gmp_integer_to_native(&gmp_x);
        let native_u32 = x.to_u32();
        assert_eq!(gmp_x.to_u32(), native_u32);
        assert_eq!(native_integer_to_rugint(&x).to_u32(), native_u32);
        if x.sign() != Ordering::Less && x.significant_bits() <= 32 {
            assert_eq!(native::Integer::from(native_u32.unwrap()), x);
            assert_eq!(native_u32, Some(x.to_u32_wrapping()));
        } else {
            assert!(native_u32.is_none());
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
fn to_u32_wrapping_properties() {
    // x.to_u32_wrapping() is equivalent for malachite-gmp, malachite-native, and rugint.
    // x.to_u32_wrapping() + (-x.to_u32_wrapping()) = 0 mod 2^32
    // TODO relate with BitAnd
    let one_integer = |gmp_x: gmp::Integer| {
        let x = gmp_integer_to_native(&gmp_x);
        let native_u32 = x.to_u32_wrapping();
        assert_eq!(gmp_x.to_u32_wrapping(), native_u32);
        assert_eq!(native_integer_to_rugint(&x).to_u32_wrapping(), native_u32);
        assert_eq!(native_u32.wrapping_add((-&x).to_u32_wrapping()), 0);
    };

    for n in exhaustive_integers().take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in random_integers(&EXAMPLE_SEED, 32).take(LARGE_LIMIT) {
        one_integer(n);
    }
}
