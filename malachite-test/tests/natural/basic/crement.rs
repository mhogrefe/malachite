use malachite_base::crement::Crementable;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{positive_unsigneds, unsigneds_no_max};
use malachite_test::inputs::natural::{naturals, positive_naturals};

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
