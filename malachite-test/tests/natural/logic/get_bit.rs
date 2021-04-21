use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitAccess, SignificantBits};
use malachite_nz::natural::logic::bit_access::limbs_get_bit;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz_test_util::common::{natural_to_biguint, natural_to_rug_integer};
use malachite_nz_test_util::natural::logic::get_bit::num_get_bit;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_unsigned_and_small_unsigned, pairs_of_unsigned_vec_and_small_unsigned,
};
use malachite_test::inputs::natural::{naturals, pairs_of_natural_and_small_unsigned};

#[test]
fn limbs_get_bit_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_unsigned,
        |&(ref limbs, index)| {
            assert_eq!(
                Natural::from_limbs_asc(limbs).get_bit(index),
                limbs_get_bit(limbs, index)
            );
        },
    );
}

#[test]
fn get_bit_properties() {
    test_properties(pairs_of_natural_and_small_unsigned, |&(ref n, index)| {
        let bit = n.get_bit(index);
        assert_eq!(num_get_bit(&natural_to_biguint(n), index), bit);
        assert_eq!(
            natural_to_rug_integer(n).get_bit(u32::exact_from(index)),
            bit
        );

        assert_eq!(n & Natural::power_of_2(index) != 0, bit);
        assert_ne!((!n).get_bit(index), bit);
    });

    test_properties(naturals, |n| {
        let significant_bits = n.significant_bits();
        assert!(!n.get_bit(significant_bits));
        if *n != 0 {
            assert!(n.get_bit(significant_bits - 1));
        }
    });

    test_properties(
        pairs_of_unsigned_and_small_unsigned::<Limb, u64>,
        |&(u, index)| {
            assert_eq!(Natural::from(u).get_bit(index), u.get_bit(index));
        },
    );
}
