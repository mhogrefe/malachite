use std::str::FromStr;

use malachite_base::crement::Crementable;
use malachite_base::num::basic::traits::Zero;

use malachite_nz::natural::Natural;

#[test]
fn test_natural_increment() {
    let test = |u, out| {
        let mut n = Natural::from_str(u).unwrap();
        n.increment();
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);
    };
    test("0", "1");
    test("123", "124");
    test("1000000000000", "1000000000001");
}

#[test]
fn test_natural_decrement() {
    let test = |u, out| {
        let mut n = Natural::from_str(u).unwrap();
        n.decrement();
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);
    };
    test("1", "0");
    test("123", "122");
    test("1000000000000", "999999999999");
}

#[test]
#[should_panic]
fn natural_decrement_fail() {
    let mut n = Natural::ZERO;
    n.decrement();
}
