use malachite_base::crement::Crementable;
use malachite_nz::integer::Integer;
use malachite_nz::platform::SignedLimb;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::signeds_no_min;
use malachite_test::inputs::integer::integers;
use malachite_test::inputs::natural::positive_naturals;

#[test]
fn decrement_properties() {
    test_properties(integers, |n| {
        let mut mut_n = n.clone();
        mut_n.decrement();
        assert_ne!(mut_n, *n);
        mut_n.increment();
        assert_eq!(mut_n, *n);
    });

    test_properties(positive_naturals, |n| {
        let mut mut_n = n.clone();
        mut_n.decrement();

        let mut i = Integer::from(n);
        i.decrement();
        assert_eq!(i, mut_n);
    });

    test_properties(signeds_no_min::<SignedLimb>, |&i| {
        let mut mut_i = i;
        mut_i.decrement();

        let mut n = Integer::from(i);
        n.decrement();
        assert_eq!(n, mut_i);
    });
}
