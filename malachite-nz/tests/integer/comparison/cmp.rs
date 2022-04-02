use malachite_base::test_util::common::test_cmp_helper;
use malachite_base::test_util::generators::signed_pair_gen;
use malachite_nz::integer::Integer;
use malachite_nz::platform::SignedLimb;
use malachite_nz::test_util::common::{integer_to_bigint, integer_to_rug_integer};
use malachite_nz::test_util::generators::{
    integer_gen, integer_pair_gen, integer_triple_gen, natural_pair_gen,
};
use num::BigInt;
use rug;
use std::cmp::Ordering;

#[test]
fn test_ord() {
    let strings = &[
        "-1000000000001",
        "-1000000000000",
        "-999999999999",
        "-123",
        "-2",
        "-1",
        "0",
        "1",
        "2",
        "123",
        "999999999999",
        "1000000000000",
        "1000000000001",
    ];
    test_cmp_helper::<Integer>(strings);
    test_cmp_helper::<BigInt>(strings);
    test_cmp_helper::<rug::Integer>(strings);
}

#[test]
fn cmp_properties() {
    integer_pair_gen().test_properties(|(x, y)| {
        let ord = x.cmp(&y);
        assert_eq!(integer_to_bigint(&x).cmp(&integer_to_bigint(&y)), ord);
        assert_eq!(
            integer_to_rug_integer(&x).cmp(&integer_to_rug_integer(&y)),
            ord
        );
        assert_eq!(y.cmp(&x).reverse(), ord);
        assert_eq!(x == y, x.cmp(&y) == Ordering::Equal);
        assert_eq!((-y).cmp(&-x), ord);
    });

    integer_gen().test_properties(|x| {
        assert_eq!(x.cmp(&x), Ordering::Equal);
    });

    integer_triple_gen().test_properties(|(x, y, z)| {
        if x < y && y < z {
            assert!(x < z);
        } else if x > y && y > z {
            assert!(x > z);
        }
    });

    natural_pair_gen().test_properties(|(x, y)| {
        assert_eq!(Integer::from(&x).cmp(&Integer::from(&y)), x.cmp(&y));
    });

    signed_pair_gen::<SignedLimb>().test_properties(|(x, y)| {
        assert_eq!(Integer::from(x).cmp(&Integer::from(y)), x.cmp(&y));
    });
}
