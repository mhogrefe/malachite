use common::LARGE_LIMIT;
use malachite_base::num::SignificantBits;
use malachite_nz::integer::Integer;
use malachite_test::common::{integer_to_rug_integer, GenerationMode};
use malachite_test::inputs::integer::integers;
use rug;
use std::cmp::Ordering;
use std::u32;
use std::str::FromStr;

#[test]
fn test_to_u32() {
    let test = |n, out| {
        assert_eq!(Integer::from_str(n).unwrap().to_u32(), out);
        assert_eq!(rug::Integer::from_str(n).unwrap().to_u32(), out);
    };
    test("0", Some(0));
    test("123", Some(123));
    test("-123", None);
    test("1000000000000", None);
    test("-1000000000000", None);
    test("4294967295", Some(u32::MAX));
    test("4294967296", None);
}

#[test]
fn test_to_u32_wrapping() {
    let test = |n, out| {
        assert_eq!(Integer::from_str(n).unwrap().to_u32_wrapping(), out);
        assert_eq!(rug::Integer::from_str(n).unwrap().to_u32_wrapping(), out);
    };
    test("0", 0);
    test("123", 123);
    test("-123", 4_294_967_173);
    test("1000000000000", 3_567_587_328);
    test("-1000000000000", 727_379_968);
    test("4294967296", 0);
    test("4294967297", 1);
    test("-4294967296", 0);
    test("-4294967295", 1);
}

#[test]
fn to_u32_properties() {
    // x.to_u32() is equivalent for malachite and rug.
    // if 0 ≤ x < 2^32, from(x.to_u32().unwrap()) == x
    // if 0 ≤ x < 2^32, x.to_u32() == Some(x.to_u32_wrapping())
    // if x < 0 or x >= 2^32, x.to_u32().is_none()
    let one_integer = |x: Integer| {
        let result = x.to_u32();
        assert_eq!(integer_to_rug_integer(&x).to_u32(), result);
        if x.sign() != Ordering::Less && x.significant_bits() <= 32 {
            assert_eq!(Integer::from(result.unwrap()), x);
            assert_eq!(result, Some(x.to_u32_wrapping()));
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
fn to_u32_wrapping_properties() {
    // x.to_u32_wrapping() is equivalent for malachite and rug.
    // x.to_u32_wrapping() + (-x.to_u32_wrapping()) = 0 mod 2^32
    // TODO relate with BitAnd
    let one_integer = |x: Integer| {
        let result = x.to_u32_wrapping();
        assert_eq!(integer_to_rug_integer(&x).to_u32_wrapping(), result);
        assert_eq!(result.wrapping_add((-&x).to_u32_wrapping()), 0);
    };

    for n in integers(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_integer(n);
    }

    for n in integers(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_integer(n);
    }
}
