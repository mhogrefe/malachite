use std::str::FromStr;

use malachite_base::num::traits::Parity;
use malachite_nz::integer::Integer;
use malachite_nz::platform::{Limb, SignedLimb};

use common::test_properties;
use malachite_test::inputs::base::signeds;
use malachite_test::inputs::integer::integers;

#[test]
fn test_even() {
    let test = |n, out| {
        assert_eq!(Integer::from_str(n).unwrap().even(), out);
    };
    test("0", true);
    test("1", false);
    test("2", true);
    test("3", false);
    test("123", false);
    test("1000000000000", true);
    test("1000000000001", false);
    test("-1", false);
    test("-2", true);
    test("-3", false);
    test("-123", false);
    test("-1000000000000", true);
    test("-1000000000001", false);
}

#[test]
fn test_odd() {
    let test = |n, out| {
        assert_eq!(Integer::from_str(n).unwrap().odd(), out);
    };
    test("0", false);
    test("1", true);
    test("2", false);
    test("3", true);
    test("123", true);
    test("1000000000000", false);
    test("1000000000001", true);
    test("-1", true);
    test("-2", false);
    test("-3", true);
    test("-123", true);
    test("-1000000000000", false);
    test("-1000000000001", true);
}

#[test]
fn even_properties() {
    test_properties(integers, |x| {
        let even = x.even();
        assert_eq!(!x.odd(), even);
        assert_eq!((x + 1 as Limb).odd(), even);
        assert_eq!((x - 1 as Limb).odd(), even);
    });

    test_properties(signeds::<SignedLimb>, |&i| {
        assert_eq!(i.even(), Integer::from(i).even());
    });
}

#[test]
fn odd_properties() {
    test_properties(integers, |x| {
        let odd = x.odd();
        assert_eq!(!x.even(), odd);
        assert_eq!((x + 1 as Limb).even(), odd);
        assert_eq!((x - 1 as Limb).even(), odd);
    });

    test_properties(signeds::<SignedLimb>, |&i| {
        assert_eq!(i.odd(), Integer::from(i).odd());
    });
}
