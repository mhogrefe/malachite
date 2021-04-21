use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::BitAccess;
use malachite_nz::integer::Integer;
use malachite_nz::natural::logic::bit_access::limbs_clear_bit;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz_test_util::common::{natural_to_rug_integer, rug_integer_to_natural};

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{
    pairs_of_unsigned_and_small_unsigned, pairs_of_unsigned_vec_and_small_unsigned,
};
use malachite_test::inputs::natural::pairs_of_natural_and_small_unsigned;

#[test]
fn limbs_clear_bit_properties() {
    test_properties(
        pairs_of_unsigned_vec_and_small_unsigned,
        |&(ref limbs, index)| {
            let mut mut_limbs = limbs.clone();
            let mut n = Natural::from_limbs_asc(limbs);
            limbs_clear_bit(&mut mut_limbs, index);
            n.clear_bit(index);
            assert_eq!(Natural::from_limbs_asc(&mut_limbs), n);
        },
    );
}

#[test]
fn clear_bit_properties() {
    test_properties(pairs_of_natural_and_small_unsigned, |&(ref n, index)| {
        let mut mut_n = n.clone();
        mut_n.clear_bit(index);
        assert!(mut_n.is_valid());
        let result = mut_n;

        let mut rug_n = natural_to_rug_integer(n);
        rug_n.set_bit(u32::exact_from(index), false);
        assert_eq!(rug_integer_to_natural(&rug_n), result);

        let mut mut_n = n.clone();
        mut_n.assign_bit(index, false);
        assert_eq!(mut_n, result);

        assert_eq!(Integer::from(n) & !Natural::power_of_2(index), result);

        assert!(result <= *n);
        if n.get_bit(index) {
            assert_ne!(result, *n);
            let mut mut_result = result.clone();
            mut_result.set_bit(index);
            assert_eq!(mut_result, *n);
        } else {
            assert_eq!(result, *n);
        }
    });

    test_properties(
        pairs_of_unsigned_and_small_unsigned::<Limb, u64>,
        |&(u, index)| {
            let mut mut_u = u;
            mut_u.clear_bit(index);
            let mut n = Natural::from(u);
            n.clear_bit(index);
            assert_eq!(n, mut_u);
        },
    );
}
