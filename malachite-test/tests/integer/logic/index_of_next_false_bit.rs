use std::str::FromStr;

use malachite_base::num::basic::traits::{NegativeOne, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitAccess, BitScan};
use malachite_nz::integer::logic::bit_scan::limbs_index_of_next_false_bit_neg;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::SignedLimb;
use rug;

use malachite_test::common::integer_to_rug_integer;
use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_signed_and_small_unsigned, pairs_of_unsigned_vec_and_small_unsigned_var_1, unsigneds,
};
use malachite_test::inputs::integer::{integers, pairs_of_integer_and_small_unsigned};
use malachite_test::inputs::natural::pairs_of_natural_and_small_unsigned;
use malachite_test::integer::logic::index_of_next_false_bit::integer_index_of_next_false_bit_alt;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_index_of_next_false_bit_neg() {
    let test = |limbs, u, out| {
        assert_eq!(limbs_index_of_next_false_bit_neg(limbs, u), out);
    };
    test(&[1], 0, None);
    test(&[1], 100, None);
    test(&[0b100], 0, Some(0));
    test(&[0b100], 1, Some(1));
    test(&[0b100], 2, None);
    test(&[0b100], 3, None);
    test(&[0, 0b101], 0, Some(0));
    test(&[0, 0b101], 20, Some(20));
    test(&[0, 0b101], 31, Some(31));
    test(&[0, 0b101], 32, Some(34));
    test(&[0, 0b101], 33, Some(34));
    test(&[0, 0b101], 34, Some(34));
    test(&[0, 0b101], 35, None);
    test(&[0, 0b101], 100, None);
    test(&[0, 0, 0b101], 36, Some(36));
    test(&[0, 0, 0b101], 64, Some(66));
    test(&[0, 0, 0b101, 0b101], 96, Some(96));
    test(&[0, 0, 0b101, 0b101], 97, Some(98));
}

#[test]
fn test_index_of_next_false_bit() {
    let test = |n, u, out| {
        assert_eq!(
            Integer::from_str(n).unwrap().index_of_next_false_bit(u),
            out
        );
        assert_eq!(
            integer_index_of_next_false_bit_alt(&Integer::from_str(n).unwrap(), u),
            out
        );
        assert_eq!(
            rug::Integer::from_str(n)
                .unwrap()
                .find_zero(u32::exact_from(u))
                .map(|u| u64::from(u)),
            out
        );
    };
    test("0", 0, Some(0));
    test("0", 100, Some(100));
    test("47244640256", 0, Some(0));
    test("47244640256", 20, Some(20));
    test("47244640256", 31, Some(31));
    test("47244640256", 32, Some(34));
    test("47244640256", 33, Some(34));
    test("47244640256", 34, Some(34));
    test("47244640256", 35, Some(36));
    test("47244640256", 100, Some(100));
    test("680564733841876926631601309731428237312", 64, Some(64));
    test("680564733841876926631601309731428237312", 68, Some(129));

    test("-21474836480", 0, Some(0));
    test("-21474836480", 20, Some(20));
    test("-21474836480", 31, Some(31));
    test("-21474836480", 32, Some(34));
    test("-21474836480", 33, Some(34));
    test("-21474836480", 34, Some(34));
    test("-21474836480", 35, None);
    test("-21474836480", 36, None);
    test("-21474836480", 100, None);
    test("-92233720368547758080", 36, Some(36));
    test("-92233720368547758080", 64, Some(66));
    test("-396140812663555408336267509760", 96, Some(96));
    test("-396140812663555408336267509760", 97, Some(98));
}

#[test]
fn limbs_index_of_next_false_bit_neg_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_unsigned_var_1,
        |&(ref limbs, u)| {
            assert_eq!(
                limbs_index_of_next_false_bit_neg(limbs, u),
                (-Natural::from_limbs_asc(limbs)).index_of_next_false_bit(u)
            );
        },
    );
}

#[test]
fn index_of_next_false_bit_properties() {
    test_properties(pairs_of_integer_and_small_unsigned, |&(ref n, u)| {
        let result = n.index_of_next_false_bit(u);
        assert_eq!(result, integer_index_of_next_false_bit_alt(n, u));
        assert_eq!(
            integer_to_rug_integer(n)
                .find_zero(u32::exact_from(u))
                .map(|u| u64::from(u)),
            result
        );
        assert_eq!(result.is_some(), n >> u != -1);
        if let Some(result) = result {
            assert!(result >= u);
            assert!(!n.get_bit(result));
            assert_eq!(result == u, !n.get_bit(u));
        }
        assert_eq!((!n).index_of_next_true_bit(u), result);
    });

    test_properties(integers, |n| {
        assert_eq!(n.index_of_next_false_bit(0), (!n).trailing_zeros());
    });

    test_properties(unsigneds::<u64>, |&u| {
        assert_eq!(Integer::ZERO.index_of_next_false_bit(u), Some(u));
        assert_eq!(Integer::NEGATIVE_ONE.index_of_next_false_bit(u), None);
    });

    test_properties(
        pairs_of_natural_and_small_unsigned::<u64>,
        |&(ref n, index)| {
            assert_eq!(
                Integer::from(n).index_of_next_false_bit(index),
                n.index_of_next_false_bit(index)
            );
        },
    );

    test_properties(
        pairs_of_signed_and_small_unsigned::<SignedLimb, u64>,
        |&(i, index)| {
            assert_eq!(
                Integer::from(i).index_of_next_false_bit(index),
                i.index_of_next_false_bit(index)
            );
        },
    );
}
