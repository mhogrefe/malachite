use malachite_base::num::arithmetic::traits::PowerOfTwo;
use malachite_base::num::logic::traits::{BitAccess, NotAssign};
use malachite_nz::integer::logic::bit_access::{
    limbs_slice_clear_bit_neg, limbs_vec_clear_bit_neg,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_limb_vec_and_small_u64_var_3, pairs_of_unsigned_vec_and_small_unsigned_var_1,
};
use malachite_test::inputs::integer::pairs_of_integer_and_small_unsigned;

macro_rules! limbs_clear_bit_neg_helper {
    ($f:ident, $limbs:ident, $index:ident) => {
        |&(ref $limbs, $index)| {
            let mut mut_limbs = $limbs.clone();
            let mut n = -Natural::from_limbs_asc($limbs);
            $f(&mut mut_limbs, $index);
            n.clear_bit($index);
            assert_eq!(-Natural::from_limbs_asc(&mut_limbs), n);
        }
    };
}

#[test]
fn limbs_slice_clear_bit_neg_properties() {
    test_properties(
        pairs_of_limb_vec_and_small_u64_var_3,
        limbs_clear_bit_neg_helper!(limbs_slice_clear_bit_neg, limbs, index),
    );
}

#[test]
fn limbs_vec_clear_bit_neg_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_unsigned_var_1,
        limbs_clear_bit_neg_helper!(limbs_vec_clear_bit_neg, limbs, index),
    );
}

#[test]
fn clear_bit_properties() {
    test_properties(pairs_of_integer_and_small_unsigned, |&(ref n, index)| {
        let mut mut_n = n.clone();
        mut_n.clear_bit(index);
        assert!(mut_n.is_valid());
        let result = mut_n;

        let mut mut_n = n.clone();
        mut_n.assign_bit(index, false);
        assert_eq!(mut_n, result);

        assert_eq!(n & !Integer::power_of_two(index), result);

        assert!(result <= *n);
        if n.get_bit(index) {
            assert_ne!(result, *n);
            let mut mut_result = result.clone();
            mut_result.set_bit(index);
            assert_eq!(mut_result, *n);
        } else {
            assert_eq!(result, *n);
        }

        let mut mut_not_n = !n;
        mut_not_n.set_bit(index);
        mut_not_n.not_assign();
        assert_eq!(mut_not_n, result);
    });
}
