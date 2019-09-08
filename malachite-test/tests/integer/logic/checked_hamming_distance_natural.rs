use std::str::FromStr;

use malachite_base::num::basic::traits::{NegativeOne, Zero};
use malachite_base::num::logic::traits::{CheckedHammingDistance, HammingDistance};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

use common::test_properties;
use malachite_test::inputs::integer::integers;
use malachite_test::inputs::integer::pairs_of_integer_and_natural;
use malachite_test::inputs::natural::{naturals, pairs_of_naturals};

#[test]
fn test_checked_hamming_distance() {
    let test = |x, y, out| {
        assert_eq!(
            Integer::from_str(x)
                .unwrap()
                .checked_hamming_distance(&Natural::from_str(y).unwrap()),
            out
        );
        assert_eq!(
            Natural::from_str(y)
                .unwrap()
                .checked_hamming_distance(&Integer::from_str(x).unwrap()),
            out
        );
    };
    test("105", "123", Some(2));
    test("1000000000000", "0", Some(13));
    test("4294967295", "0", Some(32));
    test("4294967295", "4294967295", Some(0));
    test("4294967295", "4294967296", Some(33));
    test("1000000000000", "1000000000001", Some(1));
    test("-105", "123", None);
    test("-1000000000000", "0", None);
    test("-4294967295", "0", None);
    test("-4294967295", "4294967295", None);
    test("-4294967295", "4294967296", None);
    test("-1000000000000", "1000000000001", None);
}

#[test]
fn checked_hamming_distance_properties() {
    test_properties(pairs_of_integer_and_natural, |&(ref x, ref y)| {
        let distance = x.checked_hamming_distance(y);
        assert_eq!(y.checked_hamming_distance(x), distance);
        assert_eq!(distance == Some(0), x == y);
        //TODO assert_eq!((x ^ y).checked_count_ones(), distance);
        assert_eq!((!x).checked_hamming_distance(&!y), distance);
    });

    test_properties(integers, |n| {
        assert_eq!(
            n.checked_hamming_distance(&Natural::ZERO),
            n.checked_count_ones()
        );
        assert_eq!(
            Natural::ZERO.checked_hamming_distance(n),
            n.checked_count_ones()
        );
    });

    test_properties(naturals, |n| {
        assert_eq!(n.checked_hamming_distance(&Integer::from(n)), Some(0));
        assert_eq!(Integer::from(n).checked_hamming_distance(n), Some(0));
        assert_eq!(
            n.checked_hamming_distance(&Integer::ZERO),
            Some(n.count_ones())
        );
        assert_eq!(n.checked_hamming_distance(&Integer::NEGATIVE_ONE), None);
        assert_eq!(
            Integer::ZERO.checked_hamming_distance(n),
            Some(n.count_ones())
        );
        assert_eq!(Integer::NEGATIVE_ONE.checked_hamming_distance(n), None);
    });

    test_properties(pairs_of_naturals, |&(ref x, ref y)| {
        let distance = Some(x.hamming_distance(y));
        assert_eq!(x.checked_hamming_distance(&Integer::from(y)), distance);
        assert_eq!(Integer::from(x).checked_hamming_distance(y), distance);
    });
}
