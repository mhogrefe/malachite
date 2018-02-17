use common::test_properties;
use malachite_base::misc::Walkable;
use malachite_base::num::Zero;
use malachite_nz::natural::Natural;
use malachite_test::inputs::natural::positive_naturals;
use std::str::FromStr;

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
#[should_panic(expected = "Cannot subtract a u32 from a smaller Natural. self: 0, other: 1")]
fn natural_decrement_fail() {
    let mut n = Natural::ZERO;
    n.decrement();
}

#[test]
fn natural_decrement_properties() {
    test_properties(positive_naturals, |n| {
        let mut n_mut = n.clone();
        n_mut.decrement();
        assert_ne!(n_mut, *n);
        n_mut.increment();
        assert_eq!(n_mut, *n);
    });
}
