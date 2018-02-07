use common::{test_cmp_helper, test_properties};
use malachite_nz::natural::Natural;
use malachite_test::common::{natural_to_biguint, natural_to_rug_integer};
use malachite_test::inputs::natural::{naturals, pairs_of_naturals, triples_of_naturals};
use num::BigUint;
use rug;
use std::cmp::Ordering;

#[test]
fn test_cmp() {
    let strings = vec![
        "0",
        "1",
        "2",
        "123",
        "999999999999",
        "1000000000000",
        "1000000000001",
    ];
    test_cmp_helper::<Natural>(&strings);
    test_cmp_helper::<BigUint>(&strings);
    test_cmp_helper::<rug::Integer>(&strings);
}

#[test]
fn cmp_properties() {
    test_properties(pairs_of_naturals, |&(ref x, ref y)| {
        let ord = x.cmp(y);
        assert_eq!(natural_to_biguint(x).cmp(&natural_to_biguint(y)), ord);
        assert_eq!(
            natural_to_rug_integer(x).cmp(&natural_to_rug_integer(y)),
            ord
        );
        assert_eq!(y.cmp(x).reverse(), ord);
        assert_eq!((-y).cmp(&(-x)), ord);
    });

    test_properties(naturals, |x| {
        assert_eq!(x.cmp(x), Ordering::Equal);
    });

    test_properties(triples_of_naturals, |&(ref x, ref y, ref z)| {
        if x < y && y < z {
            assert!(x < z);
        } else if x > y && y > z {
            assert!(x > z);
        }
    });
}
