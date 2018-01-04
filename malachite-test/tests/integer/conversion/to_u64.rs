use common::LARGE_LIMIT;
use malachite_native::integer as native;
use malachite_gmp::integer as gmp;
use malachite_test::common::{gmp_integer_to_native, GenerationMode};
use malachite_test::integer::conversion::to_u64::select_inputs;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use std::cmp::Ordering;
use std::str::FromStr;

#[test]
fn test_to_u64() {
    let test = |n, out| {
        assert_eq!(native::Integer::from_str(n).unwrap().to_u64(), out);
        assert_eq!(gmp::Integer::from_str(n).unwrap().to_u64(), out);
    };
    test("0", Some(0));
    test("123", Some(123));
    test("-123", None);
    test("1000000000000", Some(1_000_000_000_000));
    test("-1000000000000", None);
    test("1000000000000000000000", None);
    test("-1000000000000000000000", None);
    test("4294967295", Some(u32::max_value().into()));
    test("4294967296", Some(u64::from(u32::max_value()) + 1));
    test("18446744073709551615", Some(u64::max_value()));
    test("18446744073709551616", None);
}

#[test]
fn test_to_u64_wrapping() {
    let test = |n, out| {
        assert_eq!(native::Integer::from_str(n).unwrap().to_u64_wrapping(), out);
        assert_eq!(gmp::Integer::from_str(n).unwrap().to_u64_wrapping(), out);
    };
    test("0", 0);
    test("123", 123);
    test("-123", 18_446_744_073_709_551_493);
    test("1000000000000", 1_000_000_000_000);
    test("-1000000000000", 18_446_743_073_709_551_616);
    test("1000000000000000000000", 3_875_820_019_684_212_736);
    test("-1000000000000000000000", 14_570_924_054_025_338_880);
    test("4294967296", u64::from(u32::max_value()) + 1);
    test("4294967297", u64::from(u32::max_value()) + 2);
    test("-4294967296", 18_446_744_069_414_584_320);
    test("-4294967295", 18_446_744_069_414_584_321);
    test("18446744073709551616", 0);
    test("18446744073709551617", 1);
    test("-18446744073709551616", 0);
    test("-18446744073709551615", 1);
}

#[test]
fn to_u64_properties() {
    // x.to_u64() is equivalent for malachite-gmp and malachite-native.
    // if 0 ≤ x < 2^64, from(x.to_u64().unwrap()) == x
    // if 0 ≤ x < 2^64, x.to_u64() == Some(x.to_u64_wrapping())
    // if x < 0 or x >= 2^64, x.to_u64().is_none()
    let one_integer = |gmp_x: gmp::Integer| {
        let x = gmp_integer_to_native(&gmp_x);
        let native_u64 = x.to_u64();
        assert_eq!(gmp_x.to_u64(), native_u64);
        if x.sign() != Ordering::Less && x.significant_bits() <= 64 {
            assert_eq!(native::Integer::from(native_u64.unwrap()), x);
            assert_eq!(native_u64, Some(x.to_u64_wrapping()));
        } else {
            assert!(native_u64.is_none());
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
fn to_u64_wrapping_properties() {
    // x.to_u64_wrapping() is equivalent for malachite-gmp and malachite-native.
    // x.to_u64_wrapping() + (-x.to_u64_wrapping()) = 0 mod 2^64
    // TODO relate with BitAnd
    let one_integer = |gmp_x: gmp::Integer| {
        let x = gmp_integer_to_native(&gmp_x);
        let native_u64 = x.to_u64_wrapping();
        assert_eq!(gmp_x.to_u64_wrapping(), native_u64);
        assert_eq!(native_u64.wrapping_add((-&x).to_u64_wrapping()), 0);
    };

    for n in select_inputs(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in select_inputs(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_integer(n);
    }
}
