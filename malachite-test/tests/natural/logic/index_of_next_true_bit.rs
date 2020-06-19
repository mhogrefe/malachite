use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitAccess, BitScan, SignificantBits};
use malachite_nz::natural::logic::bit_scan::limbs_index_of_next_true_bit;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz_test_util::common::natural_to_rug_integer;
use malachite_nz_test_util::natural::logic::index_of_next_true_bit::*;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_unsigned_and_small_unsigned, pairs_of_unsigned_vec_and_small_unsigned, unsigneds,
};
use malachite_test::inputs::natural::{naturals, pairs_of_natural_and_small_unsigned};

#[test]
fn limbs_index_of_next_true_bit_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_unsigned,
        |&(ref limbs, u)| {
            assert_eq!(
                limbs_index_of_next_true_bit(limbs, u),
                Natural::from_limbs_asc(limbs).index_of_next_true_bit(u)
            );
        },
    );
}

#[test]
fn index_of_next_true_bit_properties() {
    test_properties(pairs_of_natural_and_small_unsigned, |&(ref n, u)| {
        let result = n.index_of_next_true_bit(u);
        assert_eq!(result, natural_index_of_next_true_bit_alt(n, u));
        assert_eq!(
            natural_to_rug_integer(n)
                .find_one(u32::exact_from(u))
                .map(|u| u64::from(u)),
            result
        );
        assert_eq!(result.is_some(), u < n.significant_bits());
        if let Some(result) = result {
            assert!(result >= u);
            assert!(n.get_bit(result));
            assert_eq!(result == u, n.get_bit(u));
        }
        assert_eq!((!n).index_of_next_false_bit(u), result);
    });

    test_properties(naturals, |n| {
        assert_eq!(n.index_of_next_true_bit(0), n.trailing_zeros());
    });

    test_properties(unsigneds::<u64>, |&u| {
        assert_eq!(Natural::ZERO.index_of_next_true_bit(u), None);
    });

    test_properties(
        pairs_of_unsigned_and_small_unsigned::<Limb, u64>,
        |&(u, index)| {
            assert_eq!(
                Natural::from(u).index_of_next_true_bit(index),
                u.index_of_next_true_bit(index)
            );
        },
    );
}
