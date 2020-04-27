use malachite_base::num::conversion::traits::ExactFrom;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use num::BigInt;
use rug;

use malachite_test::common::test_properties;
use malachite_test::common::{bigint_to_integer, rug_integer_to_integer};
use malachite_test::inputs::base::{natural_signeds, signeds, unsigneds};

#[test]
fn test_from_u32() {
    let test = |u: u32, out| {
        let x = Integer::from(u);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
        assert_eq!(BigInt::from(u).to_string(), out);
        assert_eq!(rug::Integer::from(u).to_string(), out);
    };
    test(0, "0");
    test(123, "123");
    test(u32::MAX, "4294967295");
}

#[test]
fn test_from_u64() {
    let test = |u: u64, out| {
        let x = Integer::from(u);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
        assert_eq!(BigInt::from(u).to_string(), out);
        assert_eq!(rug::Integer::from(u).to_string(), out);
    };
    test(0, "0");
    test(123, "123");
    test(u64::MAX, "18446744073709551615");
}

#[test]
fn test_from_i32() {
    let test = |i: i32, out| {
        let x = Integer::from(i);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
        assert_eq!(BigInt::from(i).to_string(), out);
        assert_eq!(rug::Integer::from(i).to_string(), out);
    };
    test(0, "0");
    test(123, "123");
    test(-123, "-123");
    test(i32::MIN, "-2147483648");
    test(i32::MAX, "2147483647");
}

#[test]
fn test_from_i64() {
    let test = |i: i64, out| {
        let x = Integer::from(i);
        assert_eq!(x.to_string(), out);
        assert!(x.is_valid());
        assert_eq!(BigInt::from(i).to_string(), out);
        assert_eq!(rug::Integer::from(i).to_string(), out);
    };
    test(0, "0");
    test(123, "123");
    test(-123, "-123");
    test(i64::MIN, "-9223372036854775808");
    test(i64::MAX, "9223372036854775807");
}

macro_rules! unsigned_properties {
    ($u: ident) => {
        test_properties(unsigneds::<$u>, |&u| {
            let n = Integer::from(u);
            assert!(n.is_valid());
            assert_eq!($u::exact_from(&n), u);
            assert_eq!(Integer::from(Natural::from(u)), n);
            assert_eq!(Integer::from(u128::exact_from(u)), n);
        });
    };
}

macro_rules! signed_properties {
    ($s: ident) => {
        test_properties(signeds::<$s>, |&i| {
            let n = Integer::from(i);
            assert!(n.is_valid());
            assert_eq!($s::exact_from(&n), i);
            assert_eq!(Integer::from(i128::exact_from(i)), n);
        });

        test_properties(natural_signeds::<$s>, |&i| {
            assert_eq!(Integer::from(Natural::exact_from(i)), Integer::from(i));
        });
    };
}

#[test]
fn from_primitive_integer_properties() {
    test_properties(unsigneds::<u32>, |&u| {
        let n = Integer::from(u);
        assert_eq!(bigint_to_integer(&BigInt::from(u)), n);
        assert_eq!(rug_integer_to_integer(&rug::Integer::from(u)), n);
    });

    test_properties(unsigneds::<u64>, |&u| {
        let n = Integer::from(u);
        assert_eq!(bigint_to_integer(&BigInt::from(u)), n);
        assert_eq!(rug_integer_to_integer(&rug::Integer::from(u)), n);
    });

    test_properties(signeds::<i32>, |&i| {
        let n = Integer::from(i);
        assert_eq!(bigint_to_integer(&BigInt::from(i)), n);
        assert_eq!(rug_integer_to_integer(&rug::Integer::from(i)), n);
    });

    test_properties(signeds::<i64>, |&i| {
        let n = Integer::from(i);
        assert_eq!(bigint_to_integer(&BigInt::from(i)), n);
        assert_eq!(rug_integer_to_integer(&rug::Integer::from(i)), n);
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
