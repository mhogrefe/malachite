use common::LARGE_LIMIT;
use malachite_base::num::SignificantBits;
use malachite_nz::natural::Natural;
use malachite_test::common::GenerationMode;
use malachite_test::natural::conversion::to_u64::select_inputs;
use std::str::FromStr;
use std::u64;

#[test]
fn test_to_u64() {
    let test = |n, out| {
        assert_eq!(Natural::from_str(n).unwrap().to_u64(), out);
    };
    test("0", Some(0));
    test("123", Some(123));
    test("1000000000000000000000", None);
    test("18446744073709551615", Some(u64::MAX));
    test("18446744073709551616", None);
}

#[test]
fn test_to_u64_wrapping() {
    let test = |n, out| {
        assert_eq!(Natural::from_str(n).unwrap().to_u64_wrapping(), out);
    };
    test("0", 0);
    test("123", 123);
    test("1000000000000000000000", 3_875_820_019_684_212_736);
    test("18446744073709551616", 0);
    test("18446744073709551617", 1);
}

#[test]
fn to_u64_properties() {
    // if x < 2^64, from(x.to_u64().unwrap()) == x
    // if x < 2^64, x.to_u64() == Some(x.to_u64_wrapping())
    // if x >= 2^64, x.to_u64().is_none()
    let one_natural = |x: Natural| {
        let result = x.to_u64();
        if x.significant_bits() <= 64 {
            assert_eq!(Natural::from(result.unwrap()), x);
            assert_eq!(result, Some(x.to_u64_wrapping()));
        } else {
            assert!(result.is_none());
        }
    };

    for n in select_inputs(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_natural(n);
    }

    for n in select_inputs(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_natural(n);
    }
}

#[test]
fn to_u64_wrapping_properties() {
    // TODO relate with BitAnd
    let one_natural = |x: Natural| {
        x.to_u64_wrapping();
    };

    for n in select_inputs(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_natural(n);
    }

    for n in select_inputs(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_natural(n);
    }
}
