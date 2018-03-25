use common::test_properties;
use malachite_base::misc::{CheckedFrom, WrappingFrom};
use malachite_base::num::{PrimitiveInteger, SignificantBits};
use malachite_nz::natural::Natural;
use malachite_test::inputs::natural::naturals;
use std::str::FromStr;
use std::u64;

#[test]
fn test_u64_checked_from_natural() {
    let test = |n, out| {
        assert_eq!(u64::checked_from(&Natural::from_str(n).unwrap()), out);
    };
    test("0", Some(0));
    test("123", Some(123));
    test("1000000000000000000000", None);
    test("18446744073709551615", Some(u64::MAX));
    test("18446744073709551616", None);
}

#[test]
fn test_u64_wrapping_from_natural() {
    let test = |n, out| {
        assert_eq!(u64::wrapping_from(&Natural::from_str(n).unwrap()), out);
    };
    test("0", 0);
    test("123", 123);
    test("1000000000000000000000", 3_875_820_019_684_212_736);
    test("18446744073709551616", 0);
    test("18446744073709551617", 1);
}

#[test]
fn u64_checked_from_natural_properties() {
    test_properties(naturals, |x| {
        let result = u64::checked_from(x);
        if x.significant_bits() <= u64::from(u64::WIDTH) {
            assert_eq!(Natural::from(result.unwrap()), *x);
            assert_eq!(result, Some(u64::wrapping_from(x)));
        } else {
            assert!(result.is_none());
        }
    });
}

#[test]
fn u64_wrapping_from_natural_properties() {
    // TODO relate with BitAnd
    test_properties(naturals, |x| {
        assert_eq!(
            u64::wrapping_from(x),
            u64::checked_from(&x.mod_power_of_two_ref(u64::WIDTH)).unwrap()
        );
    });
}
