use std::str::FromStr;

use malachite_base::crement::Crementable;
use malachite_base::num::traits::Zero;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use common::test_properties;
use malachite_test::inputs::base::positive_unsigneds;
use malachite_test::inputs::natural::positive_naturals;

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

#[test]
fn natural_decrement_properties() {
    test_properties(positive_naturals, |n| {
        let mut mut_n = n.clone();
        mut_n.decrement();
        assert_ne!(mut_n, *n);
        mut_n.increment();
        assert_eq!(mut_n, *n);
    });

    test_properties(positive_unsigneds::<Limb>, |&u| {
        let mut mut_u = u;
        mut_u.decrement();

        let mut n = Natural::from(u);
        n.decrement();
        assert_eq!(n, mut_u);
    });
}
