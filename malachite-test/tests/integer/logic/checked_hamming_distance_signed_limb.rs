use std::str::FromStr;

use malachite_base::comparison::{Max, Min};
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::traits::{NegativeOne, Zero};
use malachite_base::num::logic::traits::CheckedHammingDistance;
use malachite_nz::integer::logic::checked_hamming_distance_signed_limb::*;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::{Limb, SignedLimb};

use common::test_properties;
use malachite_test::inputs::base::{pairs_of_limb_vec_and_positive_limb_var_1, signeds};
use malachite_test::inputs::integer::{
    integers, pairs_of_integer_and_signed,
    triples_of_natural_integer_natural_signed_and_natural_signed,
};
use malachite_test::integer::logic::checked_hamming_distance_signed_limb::*;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_hamming_distance_limb_neg() {
    let test = |limbs, limb, out| {
        assert_eq!(limbs_hamming_distance_limb_neg(limbs, limb), out);
    };
    test(&[2], 2, 0);
    test(&[1, 1, 1], 1, 2);
    test(&[1, 1, 1], 2, 3);
    test(&[1, 2, 3], 3, 4);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_hamming_distance_limb_neg_fail() {
    limbs_hamming_distance_limb_neg(&[], 5);
}

#[test]
fn test_checked_hamming_distance_signed_limb() {
    let test = |n, i: SignedLimb, out| {
        assert_eq!(
            Integer::from_str(n).unwrap().checked_hamming_distance(i),
            out
        );
        assert_eq!(
            integer_checked_hamming_distance_signed_limb_alt_1(&Integer::from_str(n).unwrap(), i),
            out
        );
        assert_eq!(
            integer_checked_hamming_distance_signed_limb_alt_2(&Integer::from_str(n).unwrap(), i),
            out
        );
    };
    test("105", 123, Some(2));
    test("1000000000000", 0, Some(13));
    test("-105", 123, None);
    test("-1000000000000", 0, None);
    test("105", -123, None);
    test("1000000000000", -1, None);
    test("-105", -123, Some(2));
    test("-1000000000000", -1, Some(24));
    #[cfg(feature = "32_bit_limbs")]
    {
        test("2147483647", 0, Some(u64::from(Limb::WIDTH - 1)));
        test("2147483647", SignedLimb::MAX, Some(0));
        test("-2147483647", 0, None);
        test("-2147483647", SignedLimb::MAX, None);
        test("2147483647", -1, None);
        test("2147483647", SignedLimb::MIN, None);
        test("-2147483647", -1, Some(u64::from(Limb::WIDTH - 2)));
        test("-2147483647", SignedLimb::MIN, Some(1));
    }
    #[cfg(feature = "64_bit_limbs")]
    {
        test("9223372036854775807", 0, Some(u64::from(Limb::WIDTH - 1)));
        test("9223372036854775807", SignedLimb::MAX, Some(0));
        test("-9223372036854775807", 0, None);
        test("-9223372036854775807", SignedLimb::MAX, None);
        test("9223372036854775807", -1, None);
        test("9223372036854775807", SignedLimb::MIN, None);
        test("-9223372036854775807", -1, Some(u64::from(Limb::WIDTH - 2)));
        test("-9223372036854775807", SignedLimb::MIN, Some(1));
    }
}

#[test]
fn limbs_hamming_distance_limb_neg_properties() {
    test_properties(
        pairs_of_limb_vec_and_positive_limb_var_1,
        |&(ref limbs, limb)| {
            assert_eq!(
                Some(limbs_hamming_distance_limb_neg(limbs, limb)),
                (-Natural::from_limbs_asc(limbs)).checked_hamming_distance(&-Natural::from(limb)),
            );
        },
    );
}

#[test]
fn checked_hamming_distance_signed_limb_properties() {
    test_properties(
        pairs_of_integer_and_signed,
        |&(ref n, i): &(Integer, SignedLimb)| {
            let distance = n.checked_hamming_distance(i);
            assert_eq!(i.checked_hamming_distance(n), distance);
            assert_eq!(
                integer_checked_hamming_distance_signed_limb_alt_1(n, i),
                distance
            );
            assert_eq!(
                integer_checked_hamming_distance_signed_limb_alt_2(n, i),
                distance
            );
            assert_eq!(distance == Some(0), *n == i);
            assert_eq!((n ^ i).checked_count_ones(), distance);
            assert_eq!((!n).checked_hamming_distance(&!Integer::from(i)), distance);
        },
    );

    test_properties(
        triples_of_natural_integer_natural_signed_and_natural_signed,
        |&(ref a, b, c): &(Integer, SignedLimb, SignedLimb)| {
            assert!(
                a.checked_hamming_distance(c).unwrap()
                    <= a.checked_hamming_distance(b).unwrap()
                        + Integer::from(b)
                            .checked_hamming_distance(&Integer::from(c))
                            .unwrap()
            );
            let a = !a;
            let b = !b;
            let c = !c;
            assert!(
                a.checked_hamming_distance(c).unwrap()
                    <= a.checked_hamming_distance(b).unwrap()
                        + Integer::from(b)
                            .checked_hamming_distance(&Integer::from(c))
                            .unwrap()
            );
        },
    );

    test_properties(integers, |n| {
        assert_eq!(
            n.checked_hamming_distance(0 as SignedLimb),
            n.checked_count_ones()
        );
        assert_eq!(
            n.checked_hamming_distance(-1 as SignedLimb),
            n.checked_count_zeros()
        );
    });

    test_properties(signeds, |&i: &SignedLimb| {
        let i_integer = Integer::from(i);
        assert_eq!(
            Integer::ZERO.checked_hamming_distance(i),
            i_integer.checked_count_ones()
        );
        assert_eq!(
            Integer::NEGATIVE_ONE.checked_hamming_distance(i),
            i_integer.checked_count_zeros()
        );
    });
}
