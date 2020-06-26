use malachite_base::num::conversion::traits::{
    CheckedFrom, ConvertibleFrom, ExactFrom, SaturatingFrom,
};
use malachite_base::strings::ToDebugString;
use num::BigUint;
use rug;

use malachite_nz::natural::Natural;

#[test]
fn test_from_u32() {
    let test = |u: u32, out| {
        let x = Natural::from(u);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(BigUint::from(u).to_string(), out);
        assert_eq!(rug::Integer::from(u).to_string(), out);
    };
    test(0, "0");
    test(123, "123");
    test(u32::MAX, "4294967295");
}

#[test]
fn test_from_u64() {
    let test = |u: u64, out| {
        let x = Natural::from(u);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(BigUint::from(u).to_string(), out);
        assert_eq!(rug::Integer::from(u).to_string(), out);
    };
    test(0, "0");
    test(123, "123");
    test(u64::MAX, "18446744073709551615");
}

#[test]
fn test_checked_from_i32() {
    let test = |i: i32, out| {
        let on = Natural::checked_from(i);
        assert!(on.as_ref().map_or(true, Natural::is_valid));
        assert_eq!(on.to_debug_string(), out);
    };
    test(0, "Some(0)");
    test(123, "Some(123)");
    test(-123, "None");
    test(i32::MAX, "Some(2147483647)");
    test(i32::MIN, "None");
}

#[test]
fn test_exact_from_i32() {
    let test = |i: i32, out| {
        let x = Natural::exact_from(i);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
    };
    test(0, "0");
    test(123, "123");
    test(i32::MAX, "2147483647");
}

#[test]
#[should_panic]
fn exact_from_i32_fail_1() {
    Natural::exact_from(-123i32);
}

#[test]
#[should_panic]
fn exact_from_i32_fail_2() {
    Natural::exact_from(i32::MIN);
}

#[test]
fn test_saturating_from_i32() {
    let test = |i: i32, out| {
        let x = Natural::saturating_from(i);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
    };
    test(0, "0");
    test(123, "123");
    test(-123, "0");
    test(i32::MAX, "2147483647");
    test(i32::MIN, "0");
}

#[test]
fn test_convertible_from_i32() {
    let test = |i: i32, out| {
        assert_eq!(Natural::convertible_from(i), out);
    };
    test(0, true);
    test(123, true);
    test(-123, false);
    test(i32::MAX, true);
    test(i32::MIN, false);
}

#[test]
fn test_checked_from_i64() {
    let test = |i: i64, out| {
        let on = Natural::checked_from(i);
        assert!(on.as_ref().map_or(true, Natural::is_valid));
        assert_eq!(on.to_debug_string(), out);
    };
    test(0, "Some(0)");
    test(123, "Some(123)");
    test(-123, "None");
    test(i64::MAX, "Some(9223372036854775807)");
    test(i64::MIN, "None");
}

#[test]
fn test_saturating_from_i64() {
    let test = |i: i64, out| {
        let x = Natural::saturating_from(i);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
    };
    test(0, "0");
    test(123, "123");
    test(-123, "0");
    test(i64::MAX, "9223372036854775807");
    test(i64::MIN, "0");
}

#[test]
fn test_convertible_from_i64() {
    let test = |i: i64, out| {
        assert_eq!(Natural::convertible_from(i), out);
    };
    test(0, true);
    test(123, true);
    test(-123, false);
    test(i64::MAX, true);
    test(i64::MIN, false);
}
