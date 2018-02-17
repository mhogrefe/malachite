use common::test_properties;
use malachite_base::misc::Walkable;
use malachite_nz::natural::Natural;
use malachite_test::inputs::natural::naturals;
use std::str::FromStr;

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
fn natural_increment_properties() {
    test_properties(naturals, |n| {
        let mut n_mut = n.clone();
        n_mut.increment();
        assert_ne!(n_mut, *n);
        n_mut.decrement();
        assert_eq!(n_mut, *n);
    });
}
