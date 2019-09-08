use std::str::FromStr;

use malachite_base::comparison::Max;
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::logic::traits::{CheckedHammingDistance, HammingDistance};
use malachite_nz::integer::Integer;
use malachite_nz::platform::Limb;

use common::test_properties;
use malachite_test::inputs::base::unsigneds;
use malachite_test::inputs::integer::{
    integers, pairs_of_integer_and_unsigned, triples_of_natural_integer_unsigned_and_unsigned,
};
use malachite_test::inputs::natural::pairs_of_natural_and_unsigned;
use malachite_test::integer::logic::checked_hamming_distance_limb::*;

#[test]
fn test_checked_hamming_distance_limb() {
    let test = |n, u: Limb, out| {
        assert_eq!(
            Integer::from_str(n).unwrap().checked_hamming_distance(u),
            out
        );
        assert_eq!(
            integer_checked_hamming_distance_limb_alt_1(&Integer::from_str(n).unwrap(), u),
            out
        );
        assert_eq!(
            integer_checked_hamming_distance_limb_alt_2(&Integer::from_str(n).unwrap(), u),
            out
        );
    };
    test("105", 123, Some(2));
    test("1000000000000", 0, Some(13));
    #[cfg(feature = "32_bit_limbs")]
    {
        test("4294967295", 0, Some(u64::from(Limb::WIDTH)));
        test("4294967295", Limb::MAX, Some(0));
    }
    #[cfg(not(feature = "32_bit_limbs"))]
    {
        test("18446744073709551615", 0, Some(u64::from(Limb::WIDTH)));
        test("18446744073709551615", Limb::MAX, Some(0));
    }
    test("-105", 123, None);
    test("-1000000000000", 0, None);
    test("-4294967295", 0, None);
    test("-4294967295", Limb::MAX, None);
}

#[test]
fn checked_hamming_distance_limb_properties() {
    test_properties(
        pairs_of_integer_and_unsigned,
        |&(ref n, u): &(Integer, Limb)| {
            let distance = n.checked_hamming_distance(u);
            assert_eq!(u.checked_hamming_distance(n), distance);
            assert_eq!(integer_checked_hamming_distance_limb_alt_1(n, u), distance);
            assert_eq!(integer_checked_hamming_distance_limb_alt_2(n, u), distance);
            assert_eq!(distance == Some(0), *n == u);
            assert_eq!((n ^ u).checked_count_ones(), distance);
            assert_eq!((!n).checked_hamming_distance(&!Integer::from(u)), distance);
        },
    );

    test_properties(
        triples_of_natural_integer_unsigned_and_unsigned,
        |&(ref a, b, c): &(Integer, Limb, Limb)| {
            assert!(
                a.checked_hamming_distance(c).unwrap()
                    <= a.checked_hamming_distance(b).unwrap() + b.hamming_distance(c)
            );
        },
    );

    test_properties(integers, |n| {
        assert_eq!(
            n.checked_hamming_distance(0 as Limb),
            n.checked_count_ones()
        );
    });

    test_properties(pairs_of_natural_and_unsigned::<Limb>, |&(ref n, u)| {
        assert_eq!(
            Integer::from(n).checked_hamming_distance(u),
            Some(n.hamming_distance(u))
        );
    });

    test_properties(unsigneds, |&u: &Limb| {
        assert_eq!(
            Integer::ZERO.checked_hamming_distance(u),
            Some(u64::from(u.count_ones()))
        );
    });
}
