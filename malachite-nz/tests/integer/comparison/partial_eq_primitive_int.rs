use malachite_nz::integer::Integer;
use malachite_nz_test_util::integer::comparison::partial_eq_primitive_int::*;
use num::BigInt;
use rug;
use std::str::FromStr;

#[test]
fn test_partial_eq_u32() {
    let test = |s, v: u32, out| {
        let u = Integer::from_str(s).unwrap();

        assert_eq!(u == v, out);
        assert_eq!(
            num_partial_eq_primitive(&BigInt::from_str(s).unwrap(), v),
            out
        );
        assert_eq!(rug::Integer::from_str(s).unwrap() == v, out);

        assert_eq!(v == u, out);
        assert_eq!(v == rug::Integer::from_str(s).unwrap(), out);
    };
    test("0", 0, true);
    test("0", 5, false);
    test("123", 123, true);
    test("-123", 123, false);
    test("123", 5, false);
    test("-123", 5, false);
    test("1000000000000", 123, false);
    test("-1000000000000", 123, false);
}

#[test]
fn test_partial_eq_u64() {
    let test = |s, v: u64, out| {
        let u = Integer::from_str(s).unwrap();

        assert_eq!(u == v, out);
        assert_eq!(
            num_partial_eq_primitive(&BigInt::from_str(s).unwrap(), v),
            out
        );
        assert_eq!(rug::Integer::from_str(s).unwrap() == v, out);

        assert_eq!(v == u, out);
        assert_eq!(v == rug::Integer::from_str(s).unwrap(), out);
    };
    test("0", 0, true);
    test("0", 5, false);
    test("123", 123, true);
    test("-123", 123, false);
    test("123", 5, false);
    test("-123", 5, false);
    test("1000000000000", 1000000000000, true);
    test("-1000000000000", 1000000000000, false);
    test("1000000000000", 1000000000001, false);
    test("-1000000000000", 1000000000001, false);
    test("1000000000000000000000000", 1000000000000, false);
    test("-1000000000000000000000000", 1000000000000, false);
}

#[test]
fn test_partial_eq_i32() {
    let test = |s, v: i32, out| {
        let u = Integer::from_str(s).unwrap();

        assert_eq!(u == v, out);
        assert_eq!(rug::Integer::from_str(s).unwrap() == v, out);

        assert_eq!(v == u, out);
        assert_eq!(v == rug::Integer::from_str(s).unwrap(), out);
    };
    test("0", 0, true);
    test("0", 5, false);
    test("123", 123, true);
    test("-123", -123, true);
    test("-123", 123, false);
    test("123", 5, false);
    test("-123", -5, false);
    test("1000000000000", 123, false);
    test("-1000000000000", -123, false);
}

#[test]
fn test_partial_eq_i64() {
    let test = |s, v: i64, out| {
        let u = Integer::from_str(s).unwrap();

        assert_eq!(u == v, out);
        assert_eq!(rug::Integer::from_str(s).unwrap() == v, out);

        assert_eq!(v == u, out);
        assert_eq!(v == rug::Integer::from_str(s).unwrap(), out);
    };
    test("0", 0, true);
    test("0", 5, false);
    test("123", 123, true);
    test("-123", -123, true);
    test("-123", 123, false);
    test("123", 5, false);
    test("-123", -5, false);
    test("1000000000000", 1000000000000, true);
    test("-1000000000000", -1000000000000, true);
    test("1000000000000", 1000000000001, false);
    test("-1000000000000", -1000000000001, false);
    test("1000000000000000000000000", 1000000000000, false);
    test("-1000000000000000000000000", -1000000000000, false);
}
