use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitAccess, SignificantBits};
use malachite_nz::integer::logic::bit_access::limbs_get_bit_neg;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::SignedLimb;
use malachite_nz_test_util::common::integer_to_rug_integer;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_signed_and_small_unsigned, pairs_of_unsigned_vec_and_small_unsigned_var_1,
};
use malachite_test::inputs::integer::{natural_integers, pairs_of_integer_and_small_unsigned};

#[test]
fn limbs_get_bit_neg_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_unsigned_var_1,
        |&(ref limbs, index)| {
            assert_eq!(
                (-Natural::from_limbs_asc(limbs)).get_bit(index),
                limbs_get_bit_neg(limbs, index)
            );
        },
    );
}

#[test]
fn get_bit_properties() {
    test_properties(pairs_of_integer_and_small_unsigned, |&(ref n, index)| {
        let bit = n.get_bit(index);
        assert_eq!(
            integer_to_rug_integer(n).get_bit(u32::exact_from(index)),
            bit
        );

        assert_eq!(n & Integer::power_of_2(index) != 0, bit);
        assert_eq!(!(!n).get_bit(index), bit);
    });

    test_properties(natural_integers, |n| {
        let significant_bits = n.significant_bits();
        assert!(!n.get_bit(significant_bits));
        if *n != 0 {
            assert!(n.get_bit(significant_bits - 1));
        }
    });

    test_properties(
        pairs_of_signed_and_small_unsigned::<SignedLimb, u64>,
        |&(i, index)| {
            assert_eq!(Integer::from(i).get_bit(index), i.get_bit(index));
        },
    );
}
