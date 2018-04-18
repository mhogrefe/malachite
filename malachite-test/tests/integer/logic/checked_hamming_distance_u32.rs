use common::test_properties;
use malachite_base::num::{CheckedHammingDistance, HammingDistance, Zero};
use malachite_nz::integer::Integer;
use malachite_test::inputs::base::unsigneds;
use malachite_test::inputs::integer::{integers, pairs_of_integer_and_unsigned,
                                      pairs_of_natural_integer_and_unsigned, triples_of_natural_integer_unsigned_and_unsigned};
use malachite_test::integer::logic::checked_hamming_distance_u32::*;
use std::str::FromStr;
use std::u32;

#[test]
fn test_checked_hamming_distance_u32() {
    let test = |n, u: u32, out| {
        assert_eq!(
            Integer::from_str(n).unwrap().checked_hamming_distance(u),
            out
        );
    };
    test("105", 123, Some(2));
    test("1000000000000", 0, Some(13));
    test("4294967295", 0, Some(32));
    test("4294967295", u32::MAX, Some(0));
    test("-105", 123, None);
    test("-1000000000000", 0, None);
    test("-4294967295", 0, None);
    test("-4294967295", u32::MAX, None);
}

#[test]
fn checked_hamming_distance_u32_properties() {
    test_properties(pairs_of_integer_and_unsigned, |&(ref n, u): &(Integer, u32)| {
        let distance = n.checked_hamming_distance(u);
        assert_eq!(u.checked_hamming_distance(n), distance);
        assert_eq!(integer_checked_hamming_distance_u32_alt(n, u), distance);
        assert_eq!(distance == Some(0), *n == u);
        //TODO xor
        assert_eq!((!n).checked_hamming_distance(&!Integer::from(u)), distance);
    });

    test_properties(triples_of_natural_integer_unsigned_and_unsigned, |&(ref a, b, c): &(Integer, u32, u32)| {
        assert!(a.checked_hamming_distance(c).unwrap() <= a.checked_hamming_distance(b).unwrap() + b.hamming_distance(c));
    });

    test_properties(integers, |n| {
        assert_eq!(n.checked_hamming_distance(0), n.checked_count_ones());
    });

    test_properties(pairs_of_natural_integer_and_unsigned, |&(ref n, u): &(Integer, u32)| {
        assert_eq!(
            n.checked_hamming_distance(u),
            Some(n.unsigned_abs_ref().hamming_distance(u))
        )
    });

    test_properties(unsigneds, |&u: &u32| {
        assert_eq!(
            Integer::ZERO.checked_hamming_distance(u),
            Some(u64::from(u.count_ones()))
        );
    });
}
