use common::test_properties;
use malachite_base::misc::{CheckedFrom, WrappingFrom};
use malachite_base::num::{ModPowerOfTwo, PrimitiveInteger, SignificantBits};
use malachite_nz::integer::Integer;
use malachite_test::inputs::integer::integers;
use std::cmp::Ordering;
use std::str::FromStr;
use std::{u32, u64};

#[test]
fn test_u64_checked_from_integer() {
    let test = |n, out| {
        assert_eq!(u64::checked_from(&Integer::from_str(n).unwrap()), out);
    };
    test("0", Some(0));
    test("123", Some(123));
    test("-123", None);
    test("1000000000000", Some(1_000_000_000_000));
    test("-1000000000000", None);
    test("1000000000000000000000", None);
    test("-1000000000000000000000", None);
    test("4294967295", Some(u32::MAX.into()));
    test("4294967296", Some(u64::from(u32::MAX) + 1));
    test("18446744073709551615", Some(u64::MAX));
    test("18446744073709551616", None);
}

#[test]
fn test_u64_wrapping_from_integer() {
    let test = |n, out| {
        assert_eq!(u64::wrapping_from(&Integer::from_str(n).unwrap()), out);
    };
    test("0", 0);
    test("123", 123);
    test("-123", 18_446_744_073_709_551_493);
    test("1000000000000", 1_000_000_000_000);
    test("-1000000000000", 18_446_743_073_709_551_616);
    test("1000000000000000000000", 3_875_820_019_684_212_736);
    test("-1000000000000000000000", 14_570_924_054_025_338_880);
    test("4294967296", u64::from(u32::MAX) + 1);
    test("4294967297", u64::from(u32::MAX) + 2);
    test("-4294967296", 0xffff_ffff_0000_0000);
    test("-4294967295", 18_446_744_069_414_584_321);
    test("18446744073709551616", 0);
    test("18446744073709551617", 1);
    test("-18446744073709551616", 0);
    test("-18446744073709551615", 1);
}

#[test]
fn u64_checked_from_integer_properties() {
    test_properties(integers, |x| {
        let result = u64::checked_from(x);
        if x.sign() != Ordering::Less && x.significant_bits() <= u64::from(u64::WIDTH) {
            assert_eq!(Integer::from(result.unwrap()), *x);
            assert_eq!(result, Some(u64::wrapping_from(x)));
        } else {
            assert!(result.is_none());
        }
    });
}

#[test]
fn u64_wrapping_from_integer_properties() {
    test_properties(integers, |x| {
        let result = u64::wrapping_from(x);
        assert_eq!(result.wrapping_add(u64::wrapping_from(&-x)), 0);
        assert_eq!(
            result,
            u64::checked_from(&(&x).mod_power_of_two(u64::WIDTH.into())).unwrap()
        );
    });
}
