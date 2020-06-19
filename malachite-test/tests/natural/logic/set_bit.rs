use malachite_base::num::arithmetic::traits::PowerOfTwo;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::BitAccess;
use malachite_nz::natural::logic::bit_access::{limbs_slice_set_bit, limbs_vec_set_bit};
use malachite_nz::natural::Natural;
use malachite_nz_test_util::common::{
    biguint_to_natural, natural_to_biguint, natural_to_rug_integer, rug_integer_to_natural,
};
use malachite_nz_test_util::natural::logic::set_bit::num_set_bit;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_limb_vec_and_small_u64_var_2, pairs_of_unsigned_vec_and_small_unsigned,
};
use malachite_test::inputs::natural::pairs_of_natural_and_small_unsigned;

macro_rules! limbs_set_bit_helper {
    ($f:ident, $limbs:ident, $index:ident) => {
        |&(ref $limbs, $index)| {
            let mut mut_limbs = $limbs.clone();
            let mut n = Natural::from_limbs_asc($limbs);
            $f(&mut mut_limbs, $index);
            n.set_bit($index);
            assert_eq!(Natural::from_limbs_asc(&mut_limbs), n);
        }
    };
}

#[test]
fn limbs_slice_set_bit_properties() {
    test_properties(
        pairs_of_limb_vec_and_small_u64_var_2,
        limbs_set_bit_helper!(limbs_slice_set_bit, limbs, index),
    );
}

#[test]
fn limbs_vec_set_bit_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_unsigned,
        limbs_set_bit_helper!(limbs_vec_set_bit, limbs, index),
    );
}

#[test]
fn set_bit_properties() {
    test_properties(pairs_of_natural_and_small_unsigned, |&(ref n, index)| {
        let mut mut_n = n.clone();
        mut_n.set_bit(index);
        assert!(mut_n.is_valid());
        let result = mut_n;

        let mut mut_n = n.clone();
        mut_n.assign_bit(index, true);
        assert_eq!(mut_n, result);

        let mut num_n = natural_to_biguint(n);
        num_set_bit(&mut num_n, index);
        assert_eq!(biguint_to_natural(&num_n), result);

        let mut rug_n = natural_to_rug_integer(n);
        rug_n.set_bit(u32::exact_from(index), true);
        assert_eq!(rug_integer_to_natural(&rug_n), result);

        assert_eq!(n | Natural::power_of_two(index), result);

        assert_ne!(result, 0);
        assert!(result >= *n);
        if n.get_bit(index) {
            assert_eq!(result, *n);
        } else {
            assert_ne!(result, *n);
            let mut mut_result = result.clone();
            mut_result.clear_bit(index);
            assert_eq!(mut_result, *n);
        }
    });
}
