use common::test_properties;
use malachite_base::num::{CheckedHammingDistance, HammingDistance, Zero};
use malachite_nz::integer::Integer;
use malachite_nz::natural::logic::hamming_distance_u32::limbs_hamming_distance_limb;
use malachite_nz::natural::Natural;
use malachite_test::inputs::base::{pairs_of_nonempty_unsigned_vec_and_unsigned, unsigneds};
use malachite_test::inputs::natural::{
    naturals, pairs_of_natural_and_unsigned, triples_of_natural_unsigned_and_unsigned,
};
use malachite_test::natural::logic::hamming_distance_u32::{
    natural_hamming_distance_u32_alt_1, natural_hamming_distance_u32_alt_2,
};
use std::str::FromStr;
use std::u32;

#[test]
fn test_limbs_hamming_distance_limb() {
    let test = |limbs, limb, out| {
        assert_eq!(limbs_hamming_distance_limb(limbs, limb), out);
    };
    test(&[2], 3, 1);
    test(&[1, 1, 1], 1, 2);
    test(&[1, 1, 1], 2, 4);
    test(&[1, 2, 3], 0, 4);
}

#[test]
#[should_panic(expected = "index out of bounds: the len is 0 but the index is 0")]
fn limbs_hamming_distance_limb_fail() {
    limbs_hamming_distance_limb(&[], 0);
}

#[test]
fn test_hamming_distance_u32() {
    let test = |n, u, out| {
        assert_eq!(Natural::from_str(n).unwrap().hamming_distance(u), out);
        assert_eq!(
            natural_hamming_distance_u32_alt_1(&Natural::from_str(n).unwrap(), u),
            out
        );
        assert_eq!(
            natural_hamming_distance_u32_alt_2(&Natural::from_str(n).unwrap(), u),
            out
        );
    };
    test("105", 123, 2);
    test("1000000000000", 0, 13);
    test("4294967295", 0, 32);
    test("4294967295", u32::MAX, 0);
}

#[test]
fn limbs_hamming_distance_limb_properties() {
    test_properties(
        pairs_of_nonempty_unsigned_vec_and_unsigned,
        |&(ref limbs, limb)| {
            assert_eq!(
                limbs_hamming_distance_limb(limbs, limb),
                Natural::from_limbs_asc(limbs).hamming_distance(limb)
            );
        },
    );
}

#[test]
fn hamming_distance_u32_properties() {
    test_properties(
        pairs_of_natural_and_unsigned,
        |&(ref n, u): &(Natural, u32)| {
            let distance = n.hamming_distance(u);
            assert_eq!(u.hamming_distance(n), distance);
            assert_eq!(natural_hamming_distance_u32_alt_1(n, u), distance);
            assert_eq!(natural_hamming_distance_u32_alt_2(n, u), distance);
            assert_eq!(Integer::from(n).checked_hamming_distance(u), Some(distance));
            assert_eq!(distance == 0, *n == u);
            assert_eq!((n ^ u).count_ones(), distance);
            assert_eq!(
                (!n).checked_hamming_distance(&!Natural::from(u)),
                Some(distance)
            );
        },
    );

    test_properties(
        triples_of_natural_unsigned_and_unsigned,
        |&(ref a, b, c): &(Natural, u32, u32)| {
            assert!(a.hamming_distance(c) <= a.hamming_distance(b) + b.hamming_distance(c));
        },
    );

    test_properties(naturals, |n| {
        assert_eq!(n.hamming_distance(0), n.count_ones());
    });

    test_properties(unsigneds, |&u: &u32| {
        assert_eq!(Natural::ZERO.hamming_distance(u), u64::from(u.count_ones()));
    });
}
