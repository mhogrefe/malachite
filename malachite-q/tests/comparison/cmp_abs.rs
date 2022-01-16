use malachite_base::num::arithmetic::traits::Abs;
use malachite_base::num::comparison::traits::{OrdAbs, PartialOrdAbs};
use malachite_base_test_util::common::test_custom_cmp_helper;
use malachite_nz_test_util::generators::integer_pair_gen;
use malachite_q::Rational;
use malachite_q_test_util::common::rational_to_rug_rational;
use malachite_q_test_util::generators::{rational_gen, rational_pair_gen, rational_triple_gen};
use rug;
use std::cmp::Ordering;

#[test]
fn test_ord_abs() {
    let strings = &[
        "0",
        "1237/1000000000000",
        "-123/1000000",
        "3/8",
        "-5/7",
        "1",
        "-7/5",
        "2",
        "-123",
        "999999999999",
        "-1000000000000",
        "1000000000001",
    ];
    test_custom_cmp_helper::<Rational, _>(strings, OrdAbs::cmp_abs);
    test_custom_cmp_helper::<rug::Rational, _>(strings, rug::Rational::cmp_abs);
}

#[test]
fn cmp_abs_properties() {
    rational_pair_gen().test_properties(|(x, y)| {
        let ord = x.cmp_abs(&y);
        assert_eq!(
            rational_to_rug_rational(&x).cmp_abs(&rational_to_rug_rational(&y)),
            ord
        );
        assert_eq!((&x).abs().cmp(&(&y).abs()), ord);
        assert_eq!((-x).cmp_abs(&(-y)), ord);
    });

    rational_gen().test_properties(|x| {
        assert_eq!(x.cmp_abs(&x), Ordering::Equal);
        assert_eq!(x.cmp_abs(&-&x), Ordering::Equal);
    });

    rational_triple_gen().test_properties(|(x, y, z)| {
        if x.lt_abs(&y) && y.lt_abs(&z) {
            assert!(x.lt_abs(&z));
        } else if x.gt_abs(&y) && y.gt_abs(&z) {
            assert!(x.gt_abs(&z));
        }
    });

    integer_pair_gen().test_properties(|(x, y)| {
        assert_eq!(
            Rational::from(&x).cmp_abs(&Rational::from(&y)),
            x.cmp_abs(&y)
        );
    });
}
