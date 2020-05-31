use malachite_nz::integer::Integer;
use malachite_nz::platform::SignedLimb;
use malachite_nz_test_util::common::{integer_to_rug_integer, rug_integer_to_integer};

use malachite_test::common::test_properties;
use malachite_test::inputs::base::signeds;
use malachite_test::inputs::integer::integers;
use malachite_test::inputs::natural::naturals;

#[test]
fn not_properties() {
    test_properties(integers, |x| {
        let not = !x.clone();
        assert!(not.is_valid());

        let rug_not = !integer_to_rug_integer(x);
        assert_eq!(rug_integer_to_integer(&rug_not), not);

        let not_alt = !x;
        assert!(not_alt.is_valid());

        assert_eq!(not_alt, not);

        assert_ne!(not, *x);
        assert_eq!(!&not, *x);
        assert_eq!(*x >= 0, not < 0);
    });

    test_properties(signeds::<SignedLimb>, |&i| {
        assert_eq!(!Integer::from(i), !i);
    });

    test_properties(naturals, |x| {
        assert_eq!(!Integer::from(x), !x);
    });
}
