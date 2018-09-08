use common::test_properties;
use malachite_base::num::Parity;
use malachite_nz::natural::Natural;
use malachite_test::inputs::natural::naturals;
use std::str::FromStr;

#[test]
fn test_is_even() {
    let test = |n, out| {
        assert_eq!(Natural::from_str(n).unwrap().is_even(), out);
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
fn test_is_odd() {
    let test = |n, out| {
        assert_eq!(Natural::from_str(n).unwrap().is_odd(), out);
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
fn is_even_properties() {
    test_properties(naturals, |x| {
        let is_even = x.is_even();
        assert_eq!(x % 2 == 0, is_even);
        assert_eq!(!x.is_odd(), is_even);
        assert_eq!((x + 1).is_odd(), is_even);
    });
}

#[test]
fn is_odd_properties() {
    test_properties(naturals, |x| {
        let is_odd = x.is_odd();
        assert_eq!(x % 2 != 0, is_odd);
        assert_eq!(!x.is_even(), is_odd);
        assert_eq!((x + 1).is_even(), is_odd);
    });
}
