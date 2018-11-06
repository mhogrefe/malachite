use common::test_properties;
use malachite_base::misc::CheckedFrom;
use malachite_base::num::{BitAccess, BitScan, NegativeOne, Zero};
use malachite_nz::integer::logic::bit_scan::limbs_index_of_next_true_bit_neg;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_test::common::integer_to_rug_integer;
use malachite_test::inputs::base::{
    pairs_of_signed_and_small_unsigned, pairs_of_unsigned_vec_and_small_u64_var_1, unsigneds,
};
use malachite_test::inputs::integer::{integers, pairs_of_integer_and_small_u64};
use malachite_test::integer::logic::index_of_next_true_bit::integer_index_of_next_true_bit_alt;
use rug;
use std::str::FromStr;
use std::u32;

#[test]
fn test_limbs_index_of_next_true_bit_neg() {
    let test = |limbs, u, out| {
        assert_eq!(limbs_index_of_next_true_bit_neg(limbs, u), out);
    };
    test(&[1], 0, 0);
    test(&[1], 100, 100);
    test(&[0b100], 0, 2);
    test(&[0b100], 1, 2);
    test(&[0b100], 2, 2);
    test(&[0b100], 3, 3);
    test(&[0, 0b101], 0, 32);
    test(&[0, 0b101], 20, 32);
    test(&[0, 0b101], 31, 32);
    test(&[0, 0b101], 32, 32);
    test(&[0, 0b101], 33, 33);
    test(&[0, 0b101], 34, 35);
    test(&[0, 0b101], 35, 35);
    test(&[0, 0b101], 36, 36);
    test(&[0, 0b101], 100, 100);
    test(&[0, 0, 0b101], 64, 64);
    test(&[0, 0, 0b101], 66, 67);
    test(&[0, 0, 0b101, 0b101], 96, 97);
    test(&[0, 0, 0b101, 0b101], 98, 99);
}

#[test]
fn test_index_of_next_true_bit() {
    let test = |n, u, out| {
        assert_eq!(Integer::from_str(n).unwrap().index_of_next_true_bit(u), out);
        assert_eq!(
            integer_index_of_next_true_bit_alt(&Integer::from_str(n).unwrap(), u),
            out
        );
        assert_eq!(
            rug::Integer::from_str(n)
                .unwrap()
                .find_one(u32::checked_from(u).unwrap())
                .map(|u| u64::from(u)),
            out
        );
    };
    test("0", 0, None);
    test("0", 100, None);
    test("47244640256", 0, Some(32));
    test("47244640256", 20, Some(32));
    test("47244640256", 31, Some(32));
    test("47244640256", 32, Some(32));
    test("47244640256", 33, Some(33));
    test("47244640256", 34, Some(35));
    test("47244640256", 35, Some(35));
    test("47244640256", 36, None);
    test("47244640256", 100, None);
    test("340282366925890223602069384504899796992", 91, Some(91));
    test("340282366925890223602069384504899796992", 92, Some(128));

    test("-21474836480", 0, Some(32));
    test("-21474836480", 20, Some(32));
    test("-21474836480", 31, Some(32));
    test("-21474836480", 32, Some(32));
    test("-21474836480", 33, Some(33));
    test("-21474836480", 34, Some(35));
    test("-21474836480", 35, Some(35));
    test("-21474836480", 36, Some(36));
    test("-21474836480", 100, Some(100));
    test("-92233720368547758080", 64, Some(64));
    test("-92233720368547758080", 66, Some(67));
    test("-396140812663555408336267509760", 96, Some(97));
    test("-396140812663555408336267509760", 98, Some(99));
}

#[test]
fn limbs_index_of_next_true_bit_neg_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_u64_var_1,
        |&(ref limbs, u)| {
            assert_eq!(
                Some(limbs_index_of_next_true_bit_neg(limbs, u)),
                (-Natural::from_limbs_asc(limbs)).index_of_next_true_bit(u)
            );
        },
    );
}

#[test]
fn index_of_next_true_bit_properties() {
    test_properties(pairs_of_integer_and_small_u64, |&(ref n, u)| {
        let result = n.index_of_next_true_bit(u);
        assert_eq!(result, integer_index_of_next_true_bit_alt(n, u));
        assert_eq!(
            integer_to_rug_integer(n)
                .find_one(u32::checked_from(u).unwrap())
                .map(|u| u64::from(u)),
            result
        );
        assert_eq!(result.is_some(), n >> u != 0);
        if let Some(result) = result {
            assert!(result >= u);
            assert!(n.get_bit(result));
            assert_eq!(result == u, n.get_bit(u));
        }
        assert_eq!((!n).index_of_next_false_bit(u), result);
    });

    test_properties(integers, |n| {
        assert_eq!(n.index_of_next_true_bit(0), n.trailing_zeros());
    });

    test_properties(unsigneds, |&u: &u64| {
        assert_eq!(Integer::ZERO.index_of_next_true_bit(u), None);
        assert_eq!(Integer::NEGATIVE_ONE.index_of_next_true_bit(u), Some(u));
    });

    test_properties(
        pairs_of_signed_and_small_unsigned::<i32, u64>,
        |&(i, index)| {
            assert_eq!(
                Integer::from(i).index_of_next_true_bit(index),
                i.index_of_next_true_bit(index)
            );
        },
    );
}
