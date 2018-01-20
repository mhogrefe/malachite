use common::LARGE_LIMIT;
use malachite_base::num::SignificantBits;
use malachite_nz::natural::Natural;
use malachite_test::common::{natural_to_rugint_integer, GenerationMode};
use malachite_test::natural::conversion::to_u32::select_inputs;
use rugint;
use std::str::FromStr;
use std::u32;

#[test]
fn test_to_u32() {
    let test = |n, out| {
        assert_eq!(Natural::from_str(n).unwrap().to_u32(), out);
        assert_eq!(rugint::Integer::from_str(n).unwrap().to_u32(), out);
    };
    test("0", Some(0));
    test("123", Some(123));
    test("1000000000000", None);
    test("4294967295", Some(u32::MAX));
    test("4294967296", None);
}

#[test]
fn test_to_u32_wrapping() {
    let test = |n, out| {
        assert_eq!(Natural::from_str(n).unwrap().to_u32_wrapping(), out);
        assert_eq!(rugint::Integer::from_str(n).unwrap().to_u32_wrapping(), out);
    };
    test("0", 0);
    test("123", 123);
    test("1000000000000", 3_567_587_328);
    test("4294967296", 0);
    test("4294967297", 1);
}

#[test]
fn to_u32_properties() {
    // x.to_u32() is equivalent for malachite and rugint.
    // if x < 2^32, from(x.to_u32().unwrap()) == x
    // if x < 2^32, x.to_u32() == Some(x.to_u32_wrapping())
    // if x >= 2^32, x.to_u32().is_none()
    let one_natural = |x: Natural| {
        let result = x.to_u32();
        assert_eq!(natural_to_rugint_integer(&x).to_u32(), result);
        if x.significant_bits() <= 32 {
            assert_eq!(Natural::from(result.unwrap()), x);
            assert_eq!(result, Some(x.to_u32_wrapping()));
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
fn to_u32_wrapping_properties() {
    // x.to_u32_wrapping() is equivalent for malachite and rugint.
    // TODO relate with BitAnd
    let one_natural = |x: Natural| {
        let result = x.to_u32_wrapping();
        assert_eq!(natural_to_rugint_integer(&x).to_u32_wrapping(), result);
    };

    for n in select_inputs(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_natural(n);
    }

    for n in select_inputs(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_natural(n);
    }
}
