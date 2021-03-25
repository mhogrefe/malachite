use malachite_nz::natural::Natural;
use malachite_nz_test_util::natural::comparison::partial_eq_primitive_int::*;
use num::BigUint;
use rug;
use std::str::FromStr;

#[test]
fn test_partial_eq_u32() {
    let test = |u, v: u32, out| {
        assert_eq!(Natural::from_str(u).unwrap() == v, out);
        assert_eq!(
            num_partial_eq_unsigned(&BigUint::from_str(u).unwrap(), v),
            out
        );
        assert_eq!(rug::Integer::from_str(u).unwrap() == v, out);

        assert_eq!(v == Natural::from_str(u).unwrap(), out);
        assert_eq!(v == rug::Integer::from_str(u).unwrap(), out);
    };
    test("0", 0, true);
    test("0", 5, false);
    test("123", 123, true);
    test("123", 5, false);
    test("1000000000000", 123, false);
}

#[test]
fn test_partial_eq_u64() {
    let test = |u, v: u64, out| {
        assert_eq!(Natural::from_str(u).unwrap() == v, out);
        assert_eq!(
            num_partial_eq_unsigned(&BigUint::from_str(u).unwrap(), v),
            out
        );
        assert_eq!(rug::Integer::from_str(u).unwrap() == v, out);

        assert_eq!(v == Natural::from_str(u).unwrap(), out);
        assert_eq!(v == rug::Integer::from_str(u).unwrap(), out);
    };
    test("0", 0, true);
    test("0", 5, false);
    test("123", 123, true);
    test("123", 5, false);
    test("1000000000000", 1000000000000, true);
    test("1000000000000", 1000000000001, false);
    test("1000000000000000000000000", 1000000000000, false);
}

#[test]
fn test_partial_eq_i32() {
    let test = |u, v: i32, out| {
        assert_eq!(Natural::from_str(u).unwrap() == v, out);
        assert_eq!(rug::Integer::from_str(u).unwrap() == v, out);

        assert_eq!(v == Natural::from_str(u).unwrap(), out);
        assert_eq!(v == rug::Integer::from_str(u).unwrap(), out);
    };
    test("0", 0, true);
    test("0", 5, false);
    test("123", 123, true);
    test("123", 5, false);
    test("1000000000000", 123, false);
}

#[test]
fn test_partial_eq_i64() {
    let test = |u, v: i64, out| {
        assert_eq!(Natural::from_str(u).unwrap() == v, out);
        assert_eq!(rug::Integer::from_str(u).unwrap() == v, out);

        assert_eq!(v == Natural::from_str(u).unwrap(), out);
        assert_eq!(v == rug::Integer::from_str(u).unwrap(), out);
    };
    test("0", 0, true);
    test("0", 5, false);
    test("123", 123, true);
    test("123", 5, false);
    test("1000000000000", 1000000000000, true);
    test("1000000000000", 1000000000001, false);
    test("1000000000000000000000000", 1000000000000, false);
}
