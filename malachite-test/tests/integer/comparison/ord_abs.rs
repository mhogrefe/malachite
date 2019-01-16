use common::{test_custom_cmp_helper, test_properties};
use malachite_base::num::{Abs, OrdAbs, PartialOrdAbs};
use malachite_nz::integer::Integer;
use malachite_nz::platform::SignedLimb;
use malachite_test::common::integer_to_rug_integer;
use malachite_test::inputs::base::pairs_of_signeds;
use malachite_test::inputs::integer::{integers, pairs_of_integers, triples_of_integers};
use malachite_test::inputs::natural::pairs_of_naturals;
use rug;
use std::cmp::Ordering;

#[test]
fn test_ord_abs() {
    let strings = vec![
        "0",
        "1",
        "-2",
        "123",
        "-124",
        "999999999999",
        "-1000000000000",
        "1000000000001",
    ];
    test_custom_cmp_helper::<Integer, _>(&strings, |x, y| x.cmp_abs(y));
    test_custom_cmp_helper::<rug::Integer, _>(&strings, |x, y| x.cmp_abs(y));
}

#[test]
fn cmp_properties() {
    test_properties(pairs_of_integers, |&(ref x, ref y)| {
        let ord = x.cmp_abs(y);
        assert_eq!(
            integer_to_rug_integer(x).cmp_abs(&integer_to_rug_integer(y)),
            ord
        );
        assert_eq!(x.abs().cmp(&y.abs()), ord);
        assert_eq!((-x).cmp_abs(&(-y)), ord);
    });

    test_properties(integers, |x| {
        assert_eq!(x.cmp_abs(x), Ordering::Equal);
        assert_eq!(x.cmp_abs(&-x), Ordering::Equal);
    });

    test_properties(triples_of_integers, |&(ref x, ref y, ref z)| {
        if x.lt_abs(y) && y.lt_abs(z) {
            assert!(x.lt_abs(z));
        } else if x.gt_abs(y) && y.gt_abs(z) {
            assert!(x.gt_abs(z));
        }
    });

    test_properties(pairs_of_naturals, |&(ref x, ref y)| {
        assert_eq!(Integer::from(x).cmp_abs(&Integer::from(y)), x.cmp(y));
    });

    test_properties(pairs_of_signeds::<SignedLimb>, |&(x, y)| {
        assert_eq!(Integer::from(x).cmp_abs(&Integer::from(y)), x.cmp_abs(&y));
    });
}
