use malachite_base::comparison::{Max, Min};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::{
    CheckedFrom, ConvertibleFrom, ExactFrom, SaturatingFrom,
};
use malachite_nz::natural::Natural;
use num::BigUint;
use rug;

use malachite_test::common::test_properties;
use malachite_test::common::{biguint_to_natural, rug_integer_to_natural};
use malachite_test::inputs::base::{signeds, unsigneds};

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
        assert!(on.as_ref().map_or(true, |n| n.is_valid()));
        assert_eq!(format!("{:?}", on), out);
    };
    test(0, "Some(0)");
    test(123, "Some(123)");
    test(-123, "None");
    test(i32::MAX, "Some(2147483647)");
    test(i32::MIN, "None");
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
        assert!(on.as_ref().map_or(true, |n| n.is_valid()));
        assert_eq!(format!("{:?}", on), out);
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

macro_rules! unsigned_properties {
    ($t: ident) => {
        test_properties(unsigneds::<$t>, |&u| {
            let n = Natural::from(u);
            assert!(n.is_valid());
            assert_eq!($t::exact_from(&n), u);
            assert_eq!(Natural::from(u128::exact_from(u)), n);
        });
    };
}

macro_rules! signed_properties {
    ($t: ident) => {
        test_properties(signeds::<$t>, |&i| {
            let on = Natural::checked_from(i);
            assert!(on.as_ref().map_or(true, |n| n.is_valid()));
            assert_eq!(on.is_some(), i >= 0);
            assert_eq!(Natural::convertible_from(i), i >= 0);
            let n = Natural::saturating_from(i);
            assert!(n.is_valid());
            if let Some(x) = on.as_ref() {
                assert_eq!(*x, n);
                assert_eq!($t::exact_from(x), i);
                assert_eq!(Natural::exact_from(i128::exact_from(i)), n);
            } else {
                assert_eq!(n, Natural::ZERO);
            }
        });
    };
}

#[test]
fn from_primitive_integer_properties() {
    test_properties(unsigneds::<u32>, |&u| {
        let n = Natural::from(u);
        assert_eq!(biguint_to_natural(&BigUint::from(u)), n);
        assert_eq!(rug_integer_to_natural(&rug::Integer::from(u)), n);
    });

    test_properties(unsigneds::<u64>, |&u| {
        let n = Natural::from(u);
        assert_eq!(biguint_to_natural(&BigUint::from(u)), n);
        assert_eq!(rug_integer_to_natural(&rug::Integer::from(u)), n);
    });

    unsigned_properties!(u8);
    unsigned_properties!(u16);
    unsigned_properties!(u32);
    unsigned_properties!(u64);
    unsigned_properties!(usize);

    signed_properties!(i8);
    signed_properties!(i16);
    signed_properties!(i32);
    signed_properties!(i64);
    signed_properties!(isize);
}
