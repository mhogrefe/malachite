use common::test_properties;
use malachite_base::num::SignificantBits;
use malachite_nz::integer::Integer;
use malachite_test::common::integer_to_rug_integer;
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
    test_properties(integers, |x| {
        let result = x.to_u32();
        assert_eq!(integer_to_rug_integer(x).to_u32(), result);
        if x.sign() != Ordering::Less && x.significant_bits() <= 32 {
            assert_eq!(Integer::from(result.unwrap()), *x);
            assert_eq!(result, Some(x.to_u32_wrapping()));
        } else {
            assert!(result.is_none());
        }
    });
}

#[test]
fn to_u32_wrapping_properties() {
    // TODO relate with BitAnd
    test_properties(integers, |x| {
        let result = x.to_u32_wrapping();
        assert_eq!(integer_to_rug_integer(x).to_u32_wrapping(), result);
        assert_eq!(result.wrapping_add((-x).to_u32_wrapping()), 0);
    });
}
