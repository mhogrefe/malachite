use common::test_properties;
use malachite_base::num::{CheckedHammingDistance, HammingDistance, Zero};
use malachite_nz::integer::Integer;
use malachite_nz::natural::logic::hamming_distance::{
    limbs_hamming_distance, limbs_hamming_distance_same_length,
};
use malachite_nz::natural::Natural;
use malachite_test::common::natural_to_rug_integer;
use malachite_test::inputs::base::{pairs_of_unsigned_vec_var_1, pairs_of_unsigned_vec_var_2};
use malachite_test::inputs::natural::{
    naturals, pairs_of_naturals, triples_of_natural_natural_and_unsigned, triples_of_naturals,
};
use malachite_test::natural::logic::hamming_distance::{
    natural_hamming_distance_alt_1, natural_hamming_distance_alt_2,
};
use rug;
use std::str::FromStr;

//TODO continue deduplication
#[test]
fn test_limbs_hamming_distance_same_length() {
    let test = |xs, ys, out| {
        assert_eq!(limbs_hamming_distance_same_length(xs, ys), out);
    };
    test(&[], &[], 0);
    test(&[2], &[3], 1);
    test(&[1, 1, 1], &[1, 2, 3], 3);
}

#[test]
#[should_panic(expected = "assertion failed: `(left == right)`")]
fn limbs_hamming_distance_limb_same_length_fail() {
    limbs_hamming_distance_same_length(&[1], &[1, 2, 3]);
}

#[test]
fn test_limbs_hamming_distance() {
    let test = |xs, ys, out| {
        assert_eq!(limbs_hamming_distance(xs, ys), out);
    };
    test(&[], &[], 0);
    test(&[2], &[3], 1);
    test(&[1, 1, 1], &[1, 2, 3], 3);
    test(&[], &[1, 2, 3], 4);
    test(&[1, 2, 3], &[], 4);
    test(&[1, 1, 1], &[1, 2, 3, 4], 4);
    test(&[1, 2, 3, 4], &[1, 1, 1], 4);
}

#[test]
fn test_hamming_distance() {
    let test = |x, y, out| {
        assert_eq!(
            Natural::from_str(x)
                .unwrap()
                .hamming_distance(&Natural::from_str(y).unwrap()),
            out
        );
        assert_eq!(
            natural_hamming_distance_alt_1(
                &Natural::from_str(x).unwrap(),
                &Natural::from_str(y).unwrap()
            ),
            out
        );
        assert_eq!(
            natural_hamming_distance_alt_2(
                &Natural::from_str(x).unwrap(),
                &Natural::from_str(y).unwrap()
            ),
            out
        );
        assert_eq!(
            u64::from(
                rug::Integer::from_str(x)
                    .unwrap()
                    .hamming_dist(&rug::Integer::from_str(y).unwrap())
                    .unwrap()
            ),
            out
        );
    };
    test("105", "123", 2);
    test("1000000000000", "0", 13);
    test("4294967295", "0", 32);
    test("4294967295", "4294967295", 0);
    test("4294967295", "4294967296", 33);
    test("1000000000000", "1000000000001", 1);
}

#[test]
fn limbs_hamming_distance_properties_same_length() {
    test_properties(pairs_of_unsigned_vec_var_1, |&(ref xs, ref ys)| {
        assert_eq!(
            limbs_hamming_distance_same_length(xs, ys),
            Natural::from_limbs_asc(xs).hamming_distance(&Natural::from_limbs_asc(ys))
        );
    });
}

#[test]
fn limbs_hamming_distance_properties() {
    test_properties(pairs_of_unsigned_vec_var_2, |&(ref xs, ref ys)| {
        assert_eq!(
            limbs_hamming_distance(xs, ys),
            Natural::from_limbs_asc(xs).hamming_distance(&Natural::from_limbs_asc(ys))
        );
    });
}

#[test]
fn hamming_distance_properties() {
    test_properties(pairs_of_naturals, |&(ref x, ref y)| {
        let distance = x.hamming_distance(y);
        assert_eq!(
            u64::from(
                natural_to_rug_integer(x)
                    .hamming_dist(&natural_to_rug_integer(y))
                    .unwrap()
            ),
            distance
        );
        assert_eq!(y.hamming_distance(x), distance);
        assert_eq!(natural_hamming_distance_alt_1(x, y), distance);
        assert_eq!(natural_hamming_distance_alt_2(x, y), distance);
        assert_eq!(
            Integer::from(x).checked_hamming_distance(&Integer::from(y)),
            Some(distance)
        );
        assert_eq!(distance == 0, x == y);
        assert_eq!((x ^ y).count_ones(), distance);
        assert_eq!((!x).checked_hamming_distance(&!y), Some(distance));
    });

    test_properties(
        triples_of_natural_natural_and_unsigned,
        |&(ref a, ref b, c): &(Natural, Natural, u32)| {
            assert!(a.hamming_distance(c) <= a.hamming_distance(b) + b.hamming_distance(c));
        },
    );

    test_properties(triples_of_naturals, |&(ref a, ref b, ref c)| {
        assert!(a.hamming_distance(c) <= a.hamming_distance(b) + b.hamming_distance(c));
    });

    test_properties(naturals, |n| {
        assert_eq!(n.hamming_distance(&Natural::ZERO), n.count_ones());
        assert_eq!(Natural::ZERO.hamming_distance(n), n.count_ones());
    });
}
