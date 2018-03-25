use common::test_properties;
use malachite_base::misc::{CheckedFrom, WrappingFrom};
use malachite_base::num::{PrimitiveInteger, SignificantBits};
use malachite_nz::natural::Natural;
use malachite_test::common::natural_to_rug_integer;
use malachite_test::inputs::natural::naturals;
use rug;
use std::str::FromStr;
use std::u32;

#[test]
fn test_u32_checked_from_natural() {
    let test = |n, out| {
        assert_eq!(u32::checked_from(&Natural::from_str(n).unwrap()), out);
        assert_eq!(rug::Integer::from_str(n).unwrap().to_u32(), out);
    };
    test("0", Some(0));
    test("123", Some(123));
    test("1000000000000", None);
    test("4294967295", Some(u32::MAX));
    test("4294967296", None);
}

#[test]
fn test_u32_wrapping_from_natural() {
    let test = |n, out| {
        assert_eq!(u32::wrapping_from(&Natural::from_str(n).unwrap()), out);
        assert_eq!(rug::Integer::from_str(n).unwrap().to_u32_wrapping(), out);
    };
    test("0", 0);
    test("123", 123);
    test("1000000000000", 3_567_587_328);
    test("4294967296", 0);
    test("4294967297", 1);
}

#[test]
fn u32_checked_from_natural_properties() {
    test_properties(naturals, |x| {
        let result = u32::checked_from(x);
        assert_eq!(natural_to_rug_integer(x).to_u32(), result);
        if x.significant_bits() <= u64::from(u32::WIDTH) {
            assert_eq!(Natural::from(result.unwrap()), *x);
            assert_eq!(result, Some(u32::wrapping_from(x)));
        } else {
            assert!(result.is_none());
        }
    });
}

#[test]
fn u32_wrapping_from_natural_properties() {
    // TODO relate with BitAnd
    test_properties(naturals, |x| {
        let result = u32::wrapping_from(x);
        assert_eq!(natural_to_rug_integer(x).to_u32_wrapping(), result);
        assert_eq!(
            result,
            u32::checked_from(&x.mod_power_of_two_ref(u32::WIDTH)).unwrap()
        );
    });
}
