use malachite_base::num::arithmetic::traits::Abs;
use malachite_base::num::comparison::traits::{OrdAbs, PartialOrdAbs};
use malachite_base_test_util::common::test_custom_cmp_helper;
use malachite_base_test_util::generators::signed_pair_gen;
use malachite_nz::integer::Integer;
use malachite_nz::platform::SignedLimb;
use malachite_nz_test_util::common::integer_to_rug_integer;
use malachite_nz_test_util::generators::{
    integer_gen, integer_pair_gen, integer_triple_gen, natural_pair_gen,
};
use rug;
use std::cmp::Ordering;

#[test]
fn test_ord_abs() {
    let strings =
        &["0", "1", "-2", "123", "-124", "999999999999", "-1000000000000", "1000000000001"];
    test_custom_cmp_helper::<Integer, _>(strings, OrdAbs::cmp_abs);
    test_custom_cmp_helper::<rug::Integer, _>(strings, rug::Integer::cmp_abs);
}

#[test]
fn cmp_abs_properties() {
    integer_pair_gen().test_properties(|(x, y)| {
        let ord = x.cmp_abs(&y);
        assert_eq!(
            integer_to_rug_integer(&x).cmp_abs(&integer_to_rug_integer(&y)),
            ord
        );
        assert_eq!((&x).abs().cmp(&(&y).abs()), ord);
        assert_eq!((-x).cmp_abs(&(-y)), ord);
    });

    integer_gen().test_properties(|x| {
        assert_eq!(x.cmp_abs(&x), Ordering::Equal);
        assert_eq!(x.cmp_abs(&-&x), Ordering::Equal);
    });

    integer_triple_gen().test_properties(|(x, y, z)| {
        if x.lt_abs(&y) && y.lt_abs(&z) {
            assert!(x.lt_abs(&z));
        } else if x.gt_abs(&y) && y.gt_abs(&z) {
            assert!(x.gt_abs(&z));
        }
    });

    natural_pair_gen().test_properties(|(x, y)| {
        assert_eq!(Integer::from(&x).cmp_abs(&Integer::from(&y)), x.cmp(&y));
    });

    signed_pair_gen::<SignedLimb>().test_properties(|(x, y)| {
        assert_eq!(Integer::from(x).cmp_abs(&Integer::from(y)), x.cmp_abs(&y));
    });
}
