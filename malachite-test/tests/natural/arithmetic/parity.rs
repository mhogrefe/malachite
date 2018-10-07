use common::test_properties;
use malachite_base::num::{DivisibleBy, Parity};
use malachite_nz::natural::Natural;
use malachite_test::inputs::natural::naturals;
use std::str::FromStr;

#[test]
fn test_even() {
    let test = |n, out| {
        assert_eq!(Natural::from_str(n).unwrap().even(), out);
    };
    test("0", true);
    test("1", false);
    test("2", true);
    test("3", false);
    test("123", false);
    test("1000000000000", true);
    test("1000000000001", false);
}

#[test]
fn test_odd() {
    let test = |n, out| {
        assert_eq!(Natural::from_str(n).unwrap().odd(), out);
    };
    test("0", false);
    test("1", true);
    test("2", false);
    test("3", true);
    test("123", true);
    test("1000000000000", false);
    test("1000000000001", true);
}

#[test]
fn even_properties() {
    test_properties(naturals, |x| {
        let even = x.even();
        assert_eq!(x.divisible_by(2), even);
        assert_eq!(!x.odd(), even);
        assert_eq!((x + 1).odd(), even);
    });
}

#[test]
fn odd_properties() {
    test_properties(naturals, |x| {
        let odd = x.odd();
        assert_eq!(!x.divisible_by(2), odd);
        assert_eq!(!x.even(), odd);
        assert_eq!((x + 1).even(), odd);
    });
}
