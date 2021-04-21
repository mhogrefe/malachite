use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::logic::traits::{BitAccess, NotAssign};
use malachite_nz::integer::logic::bit_access::limbs_set_bit_neg;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::pairs_of_unsigned_vec_and_small_unsigned_var_1;
use malachite_test::inputs::integer::pairs_of_integer_and_small_unsigned;

#[test]
fn limbs_set_bit_neg_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_unsigned_var_1,
        |&(ref limbs, index)| {
            let mut mut_limbs = limbs.clone();
            let mut n = -Natural::from_limbs_asc(limbs);
            limbs_set_bit_neg(&mut mut_limbs, index);
            n.set_bit(index);
            assert_eq!(-Natural::from_limbs_asc(&mut_limbs), n);
        },
    );
}

#[test]
fn set_bit_properties() {
    test_properties(pairs_of_integer_and_small_unsigned, |&(ref n, index)| {
        let mut mut_n = n.clone();
        mut_n.set_bit(index);
        assert!(mut_n.is_valid());
        let result = mut_n;

        let mut mut_n = n.clone();
        mut_n.assign_bit(index, true);
        assert_eq!(mut_n, result);

        assert_eq!(n | Integer::power_of_2(index), result);

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

        let mut mut_not_n = !n;
        mut_not_n.clear_bit(index);
        mut_not_n.not_assign();
        assert_eq!(mut_not_n, result);
    });
}
