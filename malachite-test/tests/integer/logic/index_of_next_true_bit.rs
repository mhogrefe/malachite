use malachite_base::num::basic::traits::{NegativeOne, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitAccess, BitScan};
use malachite_nz::integer::logic::bit_scan::limbs_index_of_next_true_bit_neg;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::SignedLimb;
use malachite_nz_test_util::integer::logic::index_of_next_true_bit::*;

use malachite_test::common::integer_to_rug_integer;
use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_signed_and_small_unsigned, pairs_of_unsigned_vec_and_small_unsigned_var_1, unsigneds,
};
use malachite_test::inputs::integer::{integers, pairs_of_integer_and_small_unsigned};
use malachite_test::inputs::natural::pairs_of_natural_and_small_unsigned;

#[test]
fn limbs_index_of_next_true_bit_neg_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_unsigned_var_1,
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
    test_properties(pairs_of_integer_and_small_unsigned, |&(ref n, u)| {
        let result = n.index_of_next_true_bit(u);
        assert_eq!(result, integer_index_of_next_true_bit_alt(n, u));
        assert_eq!(
            integer_to_rug_integer(n)
                .find_one(u32::exact_from(u))
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

    test_properties(unsigneds::<u64>, |&u| {
        assert_eq!(Integer::ZERO.index_of_next_true_bit(u), None);
        assert_eq!(Integer::NEGATIVE_ONE.index_of_next_true_bit(u), Some(u));
    });

    test_properties(
        pairs_of_natural_and_small_unsigned::<u64>,
        |&(ref n, index)| {
            assert_eq!(
                Integer::from(n).index_of_next_true_bit(index),
                n.index_of_next_true_bit(index)
            );
        },
    );

    test_properties(
        pairs_of_signed_and_small_unsigned::<SignedLimb, u64>,
        |&(i, index)| {
            assert_eq!(
                Integer::from(i).index_of_next_true_bit(index),
                i.index_of_next_true_bit(index)
            );
        },
    );
}
