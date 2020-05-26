use malachite_base::num::basic::traits::{NegativeOne, Zero};
use malachite_base::num::logic::traits::{CheckedHammingDistance, HammingDistance};
use malachite_nz::integer::logic::checked_hamming_distance::{
    limbs_hamming_distance_limb_neg, limbs_hamming_distance_neg,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::SignedLimb;
use malachite_nz_test_util::integer::logic::checked_hamming_distance::{
    integer_checked_hamming_distance_alt_1, integer_checked_hamming_distance_alt_2,
    rug_checked_hamming_distance,
};

use malachite_test::common::integer_to_rug_integer;
use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_signeds, pairs_of_unsigned_vec_and_positive_unsigned_var_2,
    pairs_of_unsigned_vec_var_6,
};
use malachite_test::inputs::integer::{integers, pairs_of_integers, triples_of_natural_integers};
use malachite_test::inputs::natural::pairs_of_naturals;

#[test]
fn limbs_hamming_distance_limb_neg_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_positive_unsigned_var_2,
        |&(ref limbs, limb)| {
            assert_eq!(
                Some(limbs_hamming_distance_limb_neg(limbs, limb)),
                (-Natural::from_limbs_asc(limbs)).checked_hamming_distance(&-Natural::from(limb)),
            );
        },
    );
}

#[test]
fn limbs_hamming_distance_neg_properties() {
    test_properties(pairs_of_unsigned_vec_var_6, |&(ref xs, ref ys)| {
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

    test_properties(pairs_of_signeds::<SignedLimb>, |&(x, y)| {
        assert_eq!(
            Integer::from(x).checked_hamming_distance(&Integer::from(y)),
            x.checked_hamming_distance(y)
        );
    });
}
