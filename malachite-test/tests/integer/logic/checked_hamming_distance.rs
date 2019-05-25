use std::str::FromStr;

use malachite_base::num::traits::{CheckedHammingDistance, HammingDistance, NegativeOne, Zero};
use malachite_nz::integer::logic::checked_hamming_distance::limbs_hamming_distance_neg;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use rug;

use common::test_properties;
use malachite_test::common::integer_to_rug_integer;
use malachite_test::inputs::base::pairs_of_limb_vec_var_1;
use malachite_test::inputs::integer::{
    integers, pairs_of_integers, pairs_of_natural_and_integer, triples_of_natural_integers,
};
use malachite_test::inputs::natural::pairs_of_naturals;
use malachite_test::integer::logic::checked_hamming_distance::{
    integer_checked_hamming_distance_alt_1, integer_checked_hamming_distance_alt_2,
    rug_checked_hamming_distance,
};

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_hamming_distance_neg() {
    let test = |xs, ys, out| {
        assert_eq!(limbs_hamming_distance_neg(xs, ys), out);
    };
    test(&[2], &[3], 2);
    test(&[1, 1, 1], &[1, 2, 3], 3);
    test(&[1, 1, 1], &[1, 2, 3, 4], 4);
    test(&[1, 2, 3, 4], &[1, 1, 1], 4);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_hamming_distance_neg_fail_1() {
    limbs_hamming_distance_neg(&[0, 0], &[1, 2, 3]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_hamming_distance_neg_fail_2() {
    limbs_hamming_distance_neg(&[1, 2, 3], &[0, 0]);
}

#[test]
fn test_checked_hamming_distance() {
    let test = |x, y, out| {
        assert_eq!(
            Integer::from_str(x)
                .unwrap()
                .checked_hamming_distance(&Integer::from_str(y).unwrap()),
            out
        );
        assert_eq!(
            integer_checked_hamming_distance_alt_1(
                &Integer::from_str(x).unwrap(),
                &Integer::from_str(y).unwrap(),
            ),
            out
        );
        assert_eq!(
            integer_checked_hamming_distance_alt_2(
                &Integer::from_str(x).unwrap(),
                &Integer::from_str(y).unwrap(),
            ),
            out
        );
        assert_eq!(
            rug_checked_hamming_distance(
                &rug::Integer::from_str(x).unwrap(),
                &rug::Integer::from_str(y).unwrap(),
            ),
            out
        );
    };
    test("105", "123", Some(2));
    test("1000000000000", "0", Some(13));
    test("4294967295", "0", Some(32));
    test("4294967295", "4294967295", Some(0));
    test("4294967295", "4294967296", Some(33));
    test("1000000000000", "1000000000001", Some(1));
    test("-105", "-123", Some(2));
    test("-1000000000000", "-1", Some(24));
    test("-4294967295", "-1", Some(31));
    test("-4294967295", "-4294967295", Some(0));
    test("-4294967295", "-4294967296", Some(1));
    test("-1000000000000", "-1000000000001", Some(13));
    test("-105", "123", None);
    test("-1000000000000", "0", None);
    test("-4294967295", "0", None);
    test("-4294967295", "4294967295", None);
    test("-4294967295", "4294967296", None);
    test("-1000000000000", "1000000000001", None);
    test("105", "-123", None);
    test("1000000000000", "-1", None);
    test("4294967295", "-1", None);
    test("4294967295", "-4294967295", None);
    test("4294967295", "-4294967296", None);
    test("1000000000000", "-1000000000001", None);
}

#[test]
fn limbs_hamming_distance_neg_properties() {
    test_properties(pairs_of_limb_vec_var_1, |&(ref xs, ref ys)| {
        assert_eq!(
            Some(limbs_hamming_distance_neg(xs, ys)),
            (-Natural::from_limbs_asc(xs)).checked_hamming_distance(&-Natural::from_limbs_asc(ys)),
        );
    });
}

#[test]
fn checked_hamming_distance_properties() {
    test_properties(pairs_of_integers, |&(ref x, ref y)| {
        let distance = x.checked_hamming_distance(y);
        assert_eq!(
            rug_checked_hamming_distance(&integer_to_rug_integer(x), &integer_to_rug_integer(y)),
            distance
        );
        assert_eq!(y.checked_hamming_distance(x), distance);
        assert_eq!(integer_checked_hamming_distance_alt_1(x, y), distance);
        assert_eq!(integer_checked_hamming_distance_alt_2(x, y), distance);
        assert_eq!(distance == Some(0), x == y);
        assert_eq!((x ^ y).checked_count_ones(), distance);
        assert_eq!((!x).checked_hamming_distance(&!y), distance);
    });

    test_properties(triples_of_natural_integers, |&(ref a, ref b, ref c)| {
        assert!(
            a.checked_hamming_distance(c).unwrap()
                <= a.checked_hamming_distance(b).unwrap() + b.checked_hamming_distance(c).unwrap()
        );
        let a = !a;
        let b = !b;
        let c = !c;
        assert!(
            a.checked_hamming_distance(&c).unwrap()
                <= a.checked_hamming_distance(&b).unwrap()
                    + b.checked_hamming_distance(&c).unwrap()
        );
    });

    test_properties(integers, |n| {
        assert_eq!(n.checked_hamming_distance(n), Some(0));
        assert_eq!(
            n.checked_hamming_distance(&Integer::ZERO),
            n.checked_count_ones()
        );
        assert_eq!(
            n.checked_hamming_distance(&Integer::NEGATIVE_ONE),
            n.checked_count_zeros()
        );
        assert_eq!(
            Integer::ZERO.checked_hamming_distance(n),
            n.checked_count_ones()
        );
        assert_eq!(
            Integer::NEGATIVE_ONE.checked_hamming_distance(n),
            n.checked_count_zeros()
        );
    });

    test_properties(pairs_of_naturals, |&(ref x, ref y)| {
        assert_eq!(
            Integer::from(x).checked_hamming_distance(&Integer::from(y)),
            Some(x.hamming_distance(y))
        );
    });

    test_properties(pairs_of_natural_and_integer, |&(ref x, ref y)| {
        let distance = x.checked_hamming_distance(y);
        assert_eq!(Integer::from(x).checked_hamming_distance(y), distance,);
        assert_eq!(y.checked_hamming_distance(&Integer::from(x)), distance,);
    });
}
