use std::cmp::Ordering;

use malachite_nz::integer::Integer;
use malachite_nz::platform::SignedLimb;
use malachite_nz_test_util::common::integer_to_rug_integer;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::pairs_of_signeds;
use malachite_test::inputs::integer::{integers, pairs_of_integers, triples_of_integers};
use malachite_test::inputs::natural::pairs_of_naturals;

#[test]
fn cmp_properties() {
    test_properties(pairs_of_integers, |&(ref x, ref y)| {
        let ord = x.cmp(y);
        assert_eq!(
            integer_to_rug_integer(x).cmp(&integer_to_rug_integer(y)),
            ord
        );
        assert_eq!(y.cmp(x).reverse(), ord);
        assert_eq!((-y).cmp(&-x), ord);
    });

    test_properties(integers, |x| {
        assert_eq!(x.cmp(x), Ordering::Equal);
    });

    test_properties(triples_of_integers, |&(ref x, ref y, ref z)| {
        if x < y && y < z {
            assert!(x < z);
        } else if x > y && y > z {
            assert!(x > z);
        }
    });

    test_properties(pairs_of_naturals, |&(ref x, ref y)| {
        assert_eq!(Integer::from(x).cmp(&Integer::from(y)), x.cmp(y));
    });

    test_properties(pairs_of_signeds::<SignedLimb>, |&(x, y)| {
        assert_eq!(Integer::from(x).cmp(&Integer::from(y)), x.cmp(&y));
    });
}
