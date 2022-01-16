use malachite_base_test_util::common::test_cmp_helper;
use malachite_nz_test_util::generators::integer_pair_gen;
use malachite_q::Rational;
use malachite_q_test_util::common::{rational_to_bigrational, rational_to_rug_rational};
use malachite_q_test_util::generators::{rational_gen, rational_pair_gen, rational_triple_gen};
use num::BigRational;
use std::cmp::Ordering;

#[test]
fn test_cmp() {
    let strings = &[
        "-1000000000001",
        "-1000000000000",
        "-999999999999",
        "-123",
        "-2",
        "-7/5",
        "-1",
        "-5/7",
        "-3/8",
        "-123/1000000",
        "-1237/1000000000000",
        "0",
        "1237/1000000000000",
        "123/1000000",
        "3/8",
        "5/7",
        "1",
        "7/5",
        "2",
        "123",
        "999999999999",
        "1000000000000",
        "1000000000001",
    ];
    test_cmp_helper::<Rational>(strings);
    test_cmp_helper::<BigRational>(strings);
    test_cmp_helper::<rug::Rational>(strings);
}

#[test]
fn cmp_properties() {
    rational_pair_gen().test_properties(|(x, y)| {
        let ord = x.cmp(&y);
        assert_eq!(
            rational_to_bigrational(&x).cmp(&rational_to_bigrational(&y)),
            ord,
        );
        assert_eq!(
            rational_to_rug_rational(&x).cmp(&rational_to_rug_rational(&y)),
            ord,
        );
        assert_eq!(y.cmp(&x).reverse(), ord);
        assert_eq!(x == y, x.cmp(&y) == Ordering::Equal);
        assert_eq!((-y).cmp(&-x), ord);
    });

    rational_gen().test_properties(|x| {
        assert_eq!(x.cmp(&x), Ordering::Equal);
    });

    rational_triple_gen().test_properties(|(x, y, z)| {
        if x < y && y < z {
            assert!(x < z);
        } else if x > y && y > z {
            assert!(x > z);
        }
    });

    integer_pair_gen().test_properties(|(x, y)| {
        assert_eq!(Rational::from(&x).cmp(&Rational::from(&y)), x.cmp(&y));
    });
}
