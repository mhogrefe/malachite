use common::test_properties;
use malachite_base::num::{CheckedHammingDistance, NegativeOne, Zero};
use malachite_nz::integer::logic::checked_hamming_distance_i32::limbs_hamming_distance_limb_neg;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_test::inputs::base::{signeds, pairs_of_u32_vec_and_positive_u32_var_1};
use malachite_test::inputs::integer::{integers, pairs_of_integer_and_signed, triples_of_natural_integer_natural_signed_and_natural_signed};
use malachite_test::integer::logic::checked_hamming_distance_i32::*;
use std::str::FromStr;
use std::i32;

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

#[test]
#[should_panic(expected = "index out of bounds: the len is 0 but the index is 0")]
fn limbs_hamming_distance_limb_neg_fail() {
    limbs_hamming_distance_limb_neg(&[], 5);
}

#[test]
fn test_checked_hamming_distance_i32() {
    let test = |n, i: i32, out| {
        assert_eq!(
            Integer::from_str(n).unwrap().checked_hamming_distance(i),
            out
        );
    };
    test("105", 123, Some(2));
    test("1000000000000", 0, Some(13));
    test("2147483647", 0, Some(31));
    test("2147483647", i32::MAX, Some(0));
    test("-105", 123, None);
    test("-1000000000000", 0, None);
    test("-2147483647", 0, None);
    test("-2147483647", i32::MAX, None);
    test("105", -123, None);
    test("1000000000000", -1, None);
    test("2147483647", -1, None);
    test("2147483647", i32::MIN, None);
    test("-105", -123, Some(2));
    test("-1000000000000", -1, Some(24));
    test("-2147483647", -1, Some(30));
    test("-2147483647", i32::MIN, Some(1));
}

#[test]
fn limbs_hamming_distance_limb_neg_properties() {
    test_properties(
        pairs_of_u32_vec_and_positive_u32_var_1,
        |&(ref limbs, limb)| {
            assert_eq!(
                Some(limbs_hamming_distance_limb_neg(limbs, limb)),
                (-Natural::from_limbs_asc(limbs)).checked_hamming_distance(&-Natural::from(limb)),
            );
        },
    );
}

#[test]
fn checked_hamming_distance_i32_properties() {
    test_properties(pairs_of_integer_and_signed, |&(ref n, i): &(Integer, i32)| {
        let distance = n.checked_hamming_distance(i);
        assert_eq!(i.checked_hamming_distance(n), distance);
        assert_eq!(integer_checked_hamming_distance_i32_alt(n, i), distance);
        assert_eq!(distance == Some(0), *n == i);
        //TODO xor
        assert_eq!((!n).checked_hamming_distance(&!Integer::from(i)), distance);
    });

    test_properties(triples_of_natural_integer_natural_signed_and_natural_signed, |&(ref a, b, c): &(Integer, i32, i32)| {
        assert!(a.checked_hamming_distance(c).unwrap() <= a.checked_hamming_distance(b).unwrap() + Integer::from(b).checked_hamming_distance(&Integer::from(c)).unwrap());
        let a = !a;
        let b = !b;
        let c = !c;
        assert!(a.checked_hamming_distance(c).unwrap() <= a.checked_hamming_distance(b).unwrap() + Integer::from(b).checked_hamming_distance(&Integer::from(c)).unwrap());
    });

    test_properties(integers, |n| {
        assert_eq!(n.checked_hamming_distance(0), n.checked_count_ones());
        assert_eq!(n.checked_hamming_distance(-1), n.checked_count_zeros());
    });

    test_properties(signeds, |&i: &i32| {
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
