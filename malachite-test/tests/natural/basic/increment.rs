use std::str::FromStr;

use malachite_base::crement::Crementable;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::unsigneds_no_max;
use malachite_test::inputs::natural::naturals;

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
        let mut mut_n = n.clone();
        mut_n.increment();
        assert_ne!(mut_n, *n);
        mut_n.decrement();
        assert_eq!(mut_n, *n);
    });

    test_properties(unsigneds_no_max::<Limb>, |&u| {
        let mut mut_u = u;
        mut_u.increment();

        let mut n = Natural::from(u);
        n.increment();
        assert_eq!(n, mut_u);
    });
}
